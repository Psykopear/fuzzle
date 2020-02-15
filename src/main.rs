use druid::piet::UnitPoint;
use druid::widget::{Align, Flex, Label, Padding};
use druid::{theme, AppLauncher, Color, LocalizedString, PlatformError, Widget, WindowDesc};

mod widgets;
use widgets::{AutoTextBox, Icon};

enum Apps {
    Firefox,
    Chrome,
    Terminal,
}

fn icon(app: Apps) -> impl Widget<String> {
    let data: &[u8] = match app {
        Apps::Firefox => include_bytes!("assets/firefox.png"),
        Apps::Chrome => include_bytes!("assets/chrome.png"),
        Apps::Terminal => include_bytes!("assets/terminal.png"),
    };
    Icon::new(data.to_vec())
}

fn entry(app: Apps) -> impl Widget<String> {
    let text = match app {
        Apps::Firefox => "Firefox",
        Apps::Chrome => "Chrome",
        Apps::Terminal => "Terminal",
    };

    let label = Align::vertical(UnitPoint::LEFT, Label::new(text));
    Padding::new(
        (20., 0., 0., 0.),
        Flex::row()
            .with_child(Padding::new((10., 10., 10., 10.), icon(app)), 0.14)
            .with_child(label, 1.0),
    )
}

fn ui_builder() -> impl Widget<String> {
    let input = AutoTextBox::new();
    Flex::column()
        .with_child(input, 1.5)
        .with_child(Align::centered(entry(Apps::Firefox)), 1.0)
        .with_child(Align::centered(entry(Apps::Chrome)), 1.0)
        .with_child(Align::centered(entry(Apps::Terminal)), 1.0)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((550., 280.00))
        .title(LocalizedString::new("launcherrr").with_placeholder("launcherrr".to_string()));

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::BORDERED_WIDGET_HEIGHT, 100.);
            env.set(theme::TEXT_SIZE_NORMAL, 20.);
            env.set(theme::TEXT_SIZE_LARGE, 30.);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 2.);
            env.set(
                theme::WINDOW_BACKGROUND_COLOR,
                Color::rgb8(0x39, 0x3d, 0x40),
            );
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(0x39, 0x3d, 0x40));
        })
        .use_simple_logger()
        .launch("".to_string())?;
    Ok(())
}
