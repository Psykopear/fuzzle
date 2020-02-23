//! A textbox widget that keeps focus.
use std::time::{Duration, Instant};

use druid::{
    BoxConstraints, Env, Event, EventCtx, HotKey, KeyCode, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Selector, SysMods, TimerToken, UpdateCtx, Widget,
};

use druid::kurbo::{Affine, Line, Point, RoundedRect, Size, Vec2};
use druid::piet::{
    FontBuilder, PietText, PietTextLayout, RenderContext, Text, TextLayout, TextLayoutBuilder,
    UnitPoint,
};
use druid::theme;
use druid::widget::Align;

// Things I had to make public in druid to make this work, maybe I can avoid this
use druid::{offset_for_delete_backwards, EditableText, Selection};

const BORDER_WIDTH: f64 = 0.;
const PADDING_TOP: f64 = 30.;
const PADDING_LEFT: f64 = 30.;

// we send ourselves this when we want to reset blink, which must be done in event.
const RESET_BLINK: Selector = Selector::new("druid-builtin.reset-textbox-blink");

/// A widget that allows user text input.
pub struct AutoTextBox {
    placeholder: String,
    width: f64,
    hscroll_offset: f64,
    selection: Selection,
    cursor_timer: TimerToken,
    cursor_on: bool,
}

impl AutoTextBox {
    /// Create a new AutoTextBox widget
    pub fn new() -> impl Widget<String> {
        Align::vertical(UnitPoint::CENTER, Self::raw())
    }

    /// Create a new AutoTextBox widget with no Align wrapper
    pub fn raw() -> AutoTextBox {
        Self {
            width: 0.0,
            hscroll_offset: 0.,
            selection: Selection::caret(0),
            cursor_timer: TimerToken::INVALID,
            cursor_on: true,
            placeholder: String::new(),
        }
    }

    /// Calculate the PietTextLayout from the given text, font, and font size
    fn get_layout(&self, piet_text: &mut PietText, text: &str, env: &Env) -> PietTextLayout {
        let font_name = env.get(theme::FONT_NAME);
        let font_size = env.get(theme::TEXT_SIZE_LARGE);
        // TODO: caching of both the format and the layout
        let font = piet_text
            .new_font_by_name(font_name, font_size)
            .build()
            .unwrap();

        piet_text
            .new_text_layout(&font, &text.to_string())
            .build()
            .unwrap()
    }

    /// Insert text at the cursor position.
    /// Replaces selected text if there's a selection.
    fn insert(&mut self, src: &mut String, new: &str) {
        // EditableText's edit method will panic if selection is greater than
        // src length, hence we try to constrain it.
        //
        // This is especially needed when data was modified externally.
        // TODO: perhaps this belongs in update?
        let selection = self.selection.constrain_to(src);

        src.edit(selection.range(), new);
        self.selection = Selection::caret(selection.min() + new.len());
    }

    /// Set the selection to be a caret at the given offset, if that's a valid
    /// codepoint boundary.
    fn caret_to(&mut self, text: &mut String, to: usize) {
        if text.cursor(to).is_some() {
            self.selection = Selection::caret(to)
        }
    }

    /// Return the active edge of the current selection or cursor.
    // TODO: is this the right name?
    fn cursor(&self) -> usize {
        self.selection.end
    }

    /// Delete to previous grapheme if in caret mode.
    /// Otherwise just delete everything inside the selection.
    fn delete_backward(&mut self, text: &mut String) {
        if self.selection.is_caret() {
            let cursor = self.cursor();
            let new_cursor = offset_for_delete_backwards(&self.selection, text);
            text.edit(new_cursor..cursor, "");
            self.caret_to(text, new_cursor);
        } else {
            text.edit(self.selection.range(), "");
            self.caret_to(text, self.selection.min());
        }
    }

    /// Given an offset (in bytes) of a valid grapheme cluster, return
    /// the corresponding x coordinate of that grapheme on the screen.
    fn x_for_offset(&self, layout: &PietTextLayout, offset: usize) -> f64 {
        if let Some(position) = layout.hit_test_text_position(offset) {
            position.point.x
        } else {
            //TODO: what is the correct fallback here?
            0.0
        }
    }

    /// Calculate a stateful scroll offset
    fn update_hscroll(&mut self, layout: &PietTextLayout) {
        let cursor_x = self.x_for_offset(layout, self.cursor());
        let overall_text_width = layout.width();

        let padding = PADDING_LEFT * 2.;
        if overall_text_width < self.width {
            // There's no offset if text is smaller than text box
            //
            // [***I*  ]
            // ^
            self.hscroll_offset = 0.;
        } else if cursor_x > self.width + self.hscroll_offset - padding {
            // If cursor goes past right side, bump the offset
            //       ->
            // **[****I]****
            //   ^
            self.hscroll_offset = cursor_x - self.width + padding;
        } else if cursor_x < self.hscroll_offset {
            // If cursor goes past left side, match the offset
            //    <-
            // **[I****]****
            //   ^
            self.hscroll_offset = cursor_x
        }
    }

    fn reset_cursor_blink(&mut self, ctx: &mut EventCtx) {
        self.cursor_on = true;
        let deadline = Instant::now() + Duration::from_millis(500);
        self.cursor_timer = ctx.request_timer(deadline);
    }
}

impl Widget<String> for AutoTextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        // Guard against external changes in data?
        self.selection = self.selection.constrain_to(data);

        // Ensure this widget always has focus
        ctx.set_active(true);
        ctx.request_focus();

        match event {
            Event::Timer(id) => {
                if *id == self.cursor_timer {
                    self.cursor_on = !self.cursor_on;
                    let deadline = Instant::now()
                        + if self.cursor_on {
                            Duration::from_millis(1000)
                        } else {
                            Duration::from_millis(500)
                        };
                    self.cursor_timer = ctx.request_timer(deadline);
                    ctx.request_paint();
                }
            }
            Event::Command(cmd) if cmd.selector == RESET_BLINK => self.reset_cursor_blink(ctx),
            Event::KeyDown(key_event) => {
                match key_event {
                    // TODO: I'm ignoring commands here because if I just
                    // avoid propagating events in the delegate the app does not repaint
                    ke if ke.key_code == KeyCode::ArrowDown
                        || (HotKey::new(SysMods::Cmd, "j")).matches(ke) => {}
                    ke if ke.key_code == KeyCode::ArrowUp
                        || (HotKey::new(SysMods::Cmd, "k")).matches(ke) => {}
                    // Select all (Ctrl+A || Cmd+A)
                    k_e if (HotKey::new(SysMods::Cmd, "a")).matches(k_e) => {
                        self.selection.all(data);
                    }
                    // Backspace
                    k_e if (HotKey::new(None, KeyCode::Backspace)).matches(k_e) => {
                        self.delete_backward(data);
                        self.reset_cursor_blink(ctx);
                    }
                    // Actual typing
                    k_e if k_e.key_code.is_printable() => {
                        let incoming_text = k_e.text().unwrap_or("");
                        self.insert(data, incoming_text);
                        self.reset_cursor_blink(ctx);
                    }
                    _ => {}
                }
                let text_layout = self.get_layout(&mut ctx.text(), &data, env);
                self.update_hscroll(&text_layout);
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &String, _env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),
            // an open question: should we be able to schedule timers here?
            LifeCycle::FocusChanged(true) => ctx.submit_command(RESET_BLINK, ctx.widget_id()),
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        env: &Env,
    ) -> Size {
        let default_width = 100.0;

        if bc.is_width_bounded() {
            self.width = bc.max().width;
        } else {
            self.width = default_width;
        }

        bc.constrain((self.width, env.get(theme::BORDERED_WIDGET_HEIGHT)))
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &String, env: &Env) {
        // Guard against changes in data following `event`
        let content = if data.is_empty() {
            &self.placeholder
        } else {
            data
        };

        self.selection = self.selection.constrain_to(content);

        let font_size = env.get(theme::TEXT_SIZE_LARGE);
        let height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let selection_color = env.get(theme::SELECTION_COLOR);
        let text_color = env.get(theme::LABEL_COLOR);
        let placeholder_color = env.get(theme::PLACEHOLDER_COLOR);
        let cursor_color = env.get(theme::CURSOR_COLOR);

        // Paint the background
        let clip_rect = RoundedRect::from_origin_size(
            Point::ORIGIN,
            Size::new(self.width - BORDER_WIDTH, height).to_vec2(),
            env.get(theme::TEXTBOX_BORDER_RADIUS),
        );

        paint_ctx.fill(clip_rect, &background_color);

        // Render text, selection, and cursor inside a clip
        paint_ctx
            .with_save(|rc| {
                rc.clip(clip_rect);

                // Calculate layout
                let text_layout = self.get_layout(rc.text(), &content, env);

                // Shift everything inside the clip by the hscroll_offset
                rc.transform(Affine::translate((-self.hscroll_offset, 0.)));

                // Draw selection rect
                if !self.selection.is_caret() {
                    let (left, right) = (self.selection.min(), self.selection.max());
                    let left_offset = self.x_for_offset(&text_layout, left);
                    let right_offset = self.x_for_offset(&text_layout, right);

                    let selection_width = right_offset - left_offset;

                    let selection_pos =
                        Point::new(left_offset + PADDING_LEFT - 1., PADDING_TOP - 2.);

                    let selection_rect = RoundedRect::from_origin_size(
                        selection_pos,
                        Size::new(selection_width + 2., font_size + 4.).to_vec2(),
                        1.,
                    );
                    rc.fill(selection_rect, &selection_color);
                }

                // Layout, measure, and draw text
                let text_height = font_size * 0.8;
                let text_pos = Point::new(0.0 + PADDING_LEFT, text_height + PADDING_TOP);
                let color = if data.is_empty() {
                    &placeholder_color
                } else {
                    &text_color
                };

                rc.draw_text(&text_layout, text_pos, color);

                // Paint the cursor if focused and there's no selection
                if self.cursor_on && self.selection.is_caret() {
                    let cursor_x = self.x_for_offset(&text_layout, self.cursor());
                    let xy = text_pos + Vec2::new(cursor_x, 2. - font_size);
                    let x2y2 = xy + Vec2::new(0., font_size + 2.);
                    let line = Line::new(xy, x2y2);

                    rc.stroke(line, &cursor_color, 1.);
                }
                Ok(())
            })
            .unwrap();
    }
}
