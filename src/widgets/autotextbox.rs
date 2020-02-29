//! A textbox widget that keeps focus.
use druid::kurbo::{Affine, Line, Point, RoundedRect, Size, Vec2};
use druid::piet::{
    FontBuilder, PietText, PietTextLayout, RenderContext, Text, TextLayout, TextLayoutBuilder,
};
use druid::widget::TextBox;
use druid::{
    theme, BoxConstraints, Env, Event, EventCtx, HotKey, KeyCode, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Selector, SysMods, TimerToken, UpdateCtx, Widget,
};
use std::time::{Duration, Instant};

const PADDING_TOP: f64 = 30.;
const PADDING_LEFT: f64 = 30.;
const RESET_BLINK: Selector = Selector::new("reset-autotextbox-blink");

/// A widget that allows user text input.
pub struct AutoTextBox {
    textbox: Box<TextBox>,
    width: f64,
    hscroll_offset: f64,
    cursor_timer: TimerToken,
    cursor_on: bool,
    cursor: usize,
}

impl AutoTextBox {
    /// Create a new AutoTextBox widget
    pub fn new() -> Self {
        Self {
            textbox: Box::new(TextBox::raw()),
            width: 0.0,
            hscroll_offset: 0.,
            cursor_timer: TimerToken::INVALID,
            cursor: 0,
            cursor_on: true,
        }
    }

    fn get_layout(&self, piet_text: &mut PietText, text: &str, env: &Env) -> PietTextLayout {
        let font_name = env.get(theme::FONT_NAME);
        let font_size = 35.;
        let font = piet_text
            .new_font_by_name(font_name, font_size)
            .build()
            .unwrap();

        piet_text
            .new_text_layout(&font, &text.to_string())
            .build()
            .unwrap()
    }

    /// Given an offset (in bytes) of a valid grapheme cluster, return
    /// the corresponding x coordinate of that grapheme on the screen.
    fn x_for_offset(&self, layout: &PietTextLayout, offset: usize) -> f64 {
        if let Some(position) = layout.hit_test_text_position(offset) {
            position.point.x
        } else {
            0.0
        }
    }

    /// Calculate a stateful scroll offset
    fn update_hscroll(&mut self, layout: &PietTextLayout) {
        let cursor_x = self.x_for_offset(layout, self.cursor);
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
        match event {
            Event::WindowConnected => {
                ctx.submit_command(RESET_BLINK, ctx.widget_id());
                ctx.set_active(true);
                ctx.request_focus();
            }
            Event::KeyDown(key_event) => {
                match key_event {
                    // TODO: Ignoring commands here because if I just
                    //       avoid propagating events in the delegate
                    //       the app does not repaint, but that means
                    //       autotextbox need to know about global keybindings
                    ke if ke.key_code == KeyCode::ArrowDown
                        || ke.key_code == KeyCode::ArrowUp
                        || (HotKey::new(SysMods::Cmd, "j")).matches(ke)
                        || (HotKey::new(SysMods::Cmd, "k")).matches(ke)
                        || (HotKey::new(SysMods::Cmd, "n")).matches(ke)
                        || (HotKey::new(SysMods::Cmd, "p")).matches(ke) => {}
                    // Only allow some of the textbox events, I really only
                    // need to write and delete with backspace, no selection,
                    // movement, copy, paste or other stuff.
                    k_e if k_e.key_code.is_printable() => {
                        let incoming_text = k_e.text().unwrap_or("");
                        self.cursor = data.len() + incoming_text.len();
                        self.reset_cursor_blink(ctx);
                        self.textbox.event(ctx, event, data, env);
                    }
                    k_e if (HotKey::new(None, KeyCode::Backspace)).matches(k_e) => {
                        self.textbox.event(ctx, event, data, env);
                    }
                    _ => (),
                };

                let text_layout = self.get_layout(&mut ctx.text(), &data, env);
                self.update_hscroll(&text_layout);
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.selector == RESET_BLINK => self.reset_cursor_blink(ctx),
            Event::Timer(id) => {
                if *id == self.cursor_timer {
                    self.cursor_on = !self.cursor_on;
                    ctx.request_paint();
                    let deadline = Instant::now() + Duration::from_millis(500);
                    self.cursor_timer = ctx.request_timer(deadline);
                }
            }
            _ => (),
        };
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        self.textbox.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &String, data: &String, env: &Env) {
        self.textbox.update(ctx, old_data, data, env);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        let default_width = 100.0;
        if bc.is_width_bounded() {
            self.width = bc.max().width;
        } else {
            self.width = default_width;
        }
        self.textbox.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &String, env: &Env) {
        let font_size = 35.;
        let height = 75.;
        let text_color = env.get(theme::LABEL_COLOR);

        // Paint the background
        let clip_rect = RoundedRect::from_origin_size(
            Point::ORIGIN,
            Size::new(self.width, height).to_vec2(),
            env.get(theme::TEXTBOX_BORDER_RADIUS),
        );

        // Render text, selection, and cursor inside a clip
        paint_ctx
            .with_save(|rc| {
                rc.clip(clip_rect);

                // Calculate layout
                let text_layout = self.get_layout(rc.text(), &data, env);

                // Shift everything inside the clip by the hscroll_offset
                rc.transform(Affine::translate((-self.hscroll_offset, 0.)));

                // Layout, measure, and draw text
                let text_height = font_size * 0.8;
                let text_pos = Point::new(0.0 + PADDING_LEFT, text_height + PADDING_TOP);
                let color = &text_color;

                rc.draw_text(&text_layout, text_pos, color);

                // Paint the cursor
                if self.cursor_on {
                    let cursor_x = self.x_for_offset(&text_layout, self.cursor);
                    let xy = text_pos + Vec2::new(cursor_x, 2. - font_size);
                    let x2y2 = xy + Vec2::new(0., font_size + 2.);
                    let line = Line::new(xy, x2y2);

                    rc.stroke(line, &text_color, 1.);
                }
                Ok(())
            })
            .unwrap();
    }
}
