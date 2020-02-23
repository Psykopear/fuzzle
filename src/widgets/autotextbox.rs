//! A textbox widget that keeps focus.
use druid::{
    BoxConstraints, Env, Event, EventCtx, HotKey, KeyCode, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, SysMods, UpdateCtx, Widget, WidgetPod,
};

use druid::kurbo::Size;
use druid::piet::UnitPoint;
use druid::widget::{Align, TextBox};

// const BORDER_WIDTH: f64 = 0.;
// const PADDING_TOP: f64 = 30.;
// const PADDING_LEFT: f64 = 30.;

/// A widget that allows user text input.
// pub struct AutoTextBox {
//     textbox: WidgetPod<String, Box<dyn Widget<String>>>,
// }
pub struct AutoTextBox {
    textbox: TextBox,
}

impl AutoTextBox {
    /// Create a new AutoTextBox widget
    pub fn new() -> Self {
        Self {
            textbox: TextBox::raw(),
        }
    }
}

impl Widget<String> for AutoTextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        // // Ensure this widget always has focus
        ctx.set_active(true);
        ctx.request_focus();

        match event {
            Event::KeyDown(key_event) => {
                match key_event {
                    // TODO: I'm ignoring commands here because if I just
                    // avoid propagating events in the delegate the app does not repaint
                    ke if ke.key_code == KeyCode::ArrowDown
                        || (HotKey::new(SysMods::Cmd, "j")).matches(ke) => {}
                    ke if ke.key_code == KeyCode::ArrowUp
                        || (HotKey::new(SysMods::Cmd, "k")).matches(ke) => {}
                    _ => self.textbox.event(ctx, event, data, env),
                };
            }
            _ => self.textbox.event(ctx, event, data, env),
        }
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
        self.textbox.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.textbox.paint(paint_ctx, data, env);
    }
}
