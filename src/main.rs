use druid::piet::UnitPoint;
use druid::widget::{Align, Container, Flex, Label, List, ListIter, Padding, WidgetExt};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LensExt, LocalizedString, PlatformError, Widget,
    WindowDesc,
};
use std::sync::Arc;
use std::io::BufReader;

mod widgets;
use widgets::{AutoTextBox};
// use widgets::{AutoTextBox, Icon};

#[derive(Clone, Data, PartialEq)]
struct SearchResult {
    icon_path: String,
    name: String,
    description: String,
    command: String,
}

#[derive(Clone, Lens)]
struct AppState {
    input_text: String,
    search_results: Arc<Vec<SearchResult>>,
}

// I need to implement Data here because there is no Data trait
// implementation for Vec<SearchResult> and i cannot implement it
// here since Vec is not defined in the current crate.
// But I can compare the two Vecs easily if I derive PartialEq in
// the SearchResult struct
impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.input_text == other.input_text && self.search_results == other.search_results
    }
}

// fn icon(app: Apps) -> impl Widget<String> {
//     let data: &[u8] = match app {
//         Apps::Firefox => include_bytes!("assets/firefox.png"),
//         Apps::Chrome => include_bytes!("assets/chrome.png"),
//         Apps::Terminal => include_bytes!("assets/terminal.png"),
//     };
//     Icon::new(data.to_vec())
// }

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(AutoTextBox::new().lens(AppState::input_text), 1.5)
        .with_child(
            List::new(|| {
                Padding::new(
                    (20., 0., 0., 0.),
                    Flex::row()
                    // .with_child(Icon::new(|item: &SearchResult| {
                    //     let file = std::fs::File::open(item.icon_path).unwrap();
                    //     let buffer: &[u8] = BufReader::new(file).buffer();
                    //     buffer
                    // }), 1.)
                        .with_child(
                            Flex::column()
                                .with_child(
                                    Align::vertical(
                                        UnitPoint::LEFT,
                                        Label::new(|item: &SearchResult, _env: &_| {
                                            item.name.clone()
                                        }),
                                    ),
                                    1.0,
                                )
                                .with_child(
                                    Align::vertical(
                                        UnitPoint::LEFT,
                                        Label::new(|item: &SearchResult, _env: &_| {
                                            item.description.clone()
                                        }),
                                    ),
                                    1.0,
                                ),
                            3.,
                        ),
                )
            })
            .lens(AppState::search_results),
            1.0,
        )
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((550., 280.00))
        .title(LocalizedString::new("launcherrr").with_placeholder(String::from("launcherrr")));
    let data = AppState {
        input_text: "".into(),
        // Add some example results so I can buil the UI first
        // and work on the logic later
        search_results: Arc::new(vec![
            SearchResult {
                icon_path: String::from("/home/docler/src/launcherrr/src/assets/firefox.png"),
                name: String::from("Firefox"),
                description: String::from("A web browser"),
                command: String::from("/usr/bin/firefox"),
            },
            SearchResult {
                icon_path: String::from("/home/docler/src/launcherrr/src/assets/chrome.png"),
                name: String::from("Chrome"),
                description: String::from("A slightly evilish web browser"),
                command: String::from("/usr/bin/google-chrome-stable"),
            },
            SearchResult {
                icon_path: String::from("/home/docler/src/launcherrr/src/assets/terminal.png"),
                name: String::from("Alacritty"),
                description: String::from("GPU accelerated terminal in Rust"),
                command: String::from("/usr/bin/alacritty"),
            },
        ]),
    };

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
        .launch(data)?;
    Ok(())
}
// enum Apps {
//     Firefox,
//     Chrome,
//     Terminal,
// }

// fn icon(app: Apps) -> impl Widget<String> {
//     let data: &[u8] = match app {
//         Apps::Firefox => include_bytes!("assets/firefox.png"),
//         Apps::Chrome => include_bytes!("assets/chrome.png"),
//         Apps::Terminal => include_bytes!("assets/terminal.png"),
//     };
//     Icon::new(data.to_vec())
// }

// fn entry(app: Apps) -> impl Widget<String> {
//     let text = match app {
//         Apps::Firefox => "Firefox",
//         Apps::Chrome => "Chrome",
//         Apps::Terminal => "Terminal",
//     };

//     let label = Align::vertical(UnitPoint::LEFT, Label::new(text));
//     Padding::new(
//         (20., 0., 0., 0.),
//         Flex::row()
//             .with_child(Padding::new((10., 10., 10., 10.), icon(app)), 0.14)
//             .with_child(label, 1.0),
//     )
// }

// fn ui_builder() -> impl Widget<String> {
//     let input = AutoTextBox::new();
//     Flex::column()
//         .with_child(input, 1.5)
//         .with_child(Align::centered(entry(Apps::Firefox)), 1.0)
//         .with_child(Align::centered(entry(Apps::Chrome)), 1.0)
//         .with_child(Align::centered(entry(Apps::Terminal)), 1.0)
// }
