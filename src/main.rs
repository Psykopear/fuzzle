// use druid::piet::UnitPoint;
// use druid::widget::{Container, EnvScope, Flex, Label, List, Padding, WidgetExt};
use druid::widget::{Flex, List, WidgetExt};
// use druid::{theme, AppLauncher, Color, Env, LocalizedString, PlatformError, Widget, WindowDesc};
use druid::{theme, AppLauncher, Color, LocalizedString, PlatformError, Widget, WindowDesc};

use std::sync::Arc;

mod widgets;
use widgets::{AutoTextBox, ListElement};

mod state;
use state::{AppState, SearchResult};

mod delegate;
use delegate::Delegate;

mod dirutils;

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(AutoTextBox::new().lens(AppState::input_text), 1.)
        .with_child(
            List::new(ListElement::new).lens(AppState::search_results),
            3.,
        )
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((550., 320.00))
        .resizable(false)
        .show_titlebar(false)
        .title(LocalizedString::new("fuzzle").with_placeholder(String::from("fuzzle")));
    let data = AppState {
        input_text: "".into(),
        search_results: Arc::new(vec![]),
        selected_line: 0,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .configure_env(|env, _| {
            env.set(theme::BORDERED_WIDGET_HEIGHT, 100.);
            env.set(theme::TEXT_SIZE_NORMAL, 20.);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 2.);
            env.set(
                theme::WINDOW_BACKGROUND_COLOR,
                Color::rgb8(0x39, 0x3d, 0x40),
            );
            env.set(theme::LABEL_COLOR, Color::rgb8(0xf2, 0xf2, 0xf2));
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(0x39, 0x3d, 0x40));
        })
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
