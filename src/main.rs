use druid::widget::{Align, Flex, TextBox};
use druid::{theme, AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};

fn ui_builder() -> impl Widget<String> {
    let input = TextBox::new();
    Flex::column().with_child(Align::centered(input), 1.0)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((600., 100.))
        .title(LocalizedString::new("rlaunch").with_placeholder("Rust launcher".to_string()));

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::BORDERED_WIDGET_HEIGHT, 100.);
            env.set(theme::TEXT_SIZE_NORMAL, 40.);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 15.);
        })
        .use_simple_logger()
        .launch("".to_string())?;
    Ok(())
}
