use druid::widget::{Align, Flex, TextBox};
use druid::{theme, AppLauncher, Color, LocalizedString, PlatformError, Widget, WindowDesc};

fn ui_builder() -> impl Widget<String> {
    let input = TextBox::new();
    Flex::column().with_child(Align::centered(input), 1.0)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((600., 100.))
        .title(LocalizedString::new("launcherrr").with_placeholder("launcherrr".to_string()));

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::BORDERED_WIDGET_HEIGHT, 100.);
            env.set(theme::TEXT_SIZE_NORMAL, 40.);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 2.);
            env.set(
                theme::WINDOW_BACKGROUND_COLOR,
                Color::rgb8(0x28, 0x2c, 0x34),
            );
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(0x28, 0x2c, 0x34));
        })
        .use_simple_logger()
        .launch("".to_string())?;
    Ok(())
}
