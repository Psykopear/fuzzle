use druid::piet::UnitPoint;
use druid::widget::{Align, Flex, Label, List, Padding, Scroll, WidgetExt};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, PlatformError, Widget, WindowDesc,
};
use std::sync::Arc;

mod widgets;
use widgets::{AutoTextBox, Icon};

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

fn make_ui() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let autotextbox = AutoTextBox::new().lens(AppState::input_text);
    col.add_child(autotextbox, 1.);

    let element = || {
        Padding::new(
            (25., 15., 15., 15.),
            Flex::row()
                .with_child(
                    Icon::new(|item: &SearchResult, _env: &_| item.icon_path.clone()),
                    1.,
                )
                .with_child(
                    Flex::column()
                        .with_child(
                            Label::new(|item: &SearchResult, _env: &_| item.name.clone())
                                .color(Color::rgb8(0xc2, 0xc2, 0xc2))
                                .text_align(UnitPoint::LEFT),
                            1.0,
                        )
                        .with_child(
                            Label::new(|item: &SearchResult, _env: &_| item.description.clone())
                            .color(Color::rgb8(0x72, 0x72, 0x72))
                                .text_align(UnitPoint::LEFT),
                            1.0,
                        ),
                    8.,
                ),
        )
        .fix_height(75.)
    };

    let searchresults = List::new(element).lens(AppState::search_results);
    col.add_child(searchresults, 3.);

    col
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((550., 320.00))
        .title(LocalizedString::new("launcherrr").with_placeholder(String::from("launcherrr")));
    let data = AppState {
        input_text: "".into(),
        // Add some example results so I can build the UI first
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
                description: String::from("A privacy oriented web browser"),
                command: String::from("/usr/bin/google-chrome-stable"),
            },
            SearchResult {
                icon_path: String::from("/home/docler/src/launcherrr/src/assets/godot.png"),
                name: String::from("Godot"),
                description: String::from("Open source game engine"),
                command: String::from("/usr/bin/godot"),
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
            env.set(theme::LABEL_COLOR, Color::rgb8(0xf2, 0xf2, 0xf2));
            env.set(theme::LABEL_SECONDARY_COLOR, Color::rgb8(0xa2, 0xa2, 0xa2));
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(0x39, 0x3d, 0x40));
        })
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
