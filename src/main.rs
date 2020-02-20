use druid::piet::UnitPoint;
use druid::widget::{Container, EnvScope, Flex, Label, List, Padding, WidgetExt};
use druid::{theme, AppLauncher, Color, Env, LocalizedString, PlatformError, Widget, WindowDesc};

use std::sync::Arc;

mod widgets;
use widgets::{AutoTextBox, Icon};

mod state;
use state::{AppState, SearchResult};

mod delegate;
use delegate::Delegate;

const BG_COLOR: Color = Color::rgb8(0x39, 0x3d, 0x40);
const LABEL_MAIN_COLOR: Color = Color::rgb8(0xc2, 0xc2, 0xc2);
const LABEL_SECONDARY_COLOR: Color = Color::rgb8(0x72, 0x72, 0x72);
const LIST_ELEMENT_COLOR: Color = Color::rgba8(0xff, 0xff, 0xff, 0x22);
const LIST_ELEMENT_COLOR_SELECTED: Color = Color::rgba8(0xff, 0xff, 0xff, 0x00);

fn list_element() -> impl Widget<SearchResult> {
    EnvScope::new(
        |env: &mut Env, data: &SearchResult| {
            if data.selected {
                env.set(theme::CONTAINER_BACKGROUND_COLOR, LIST_ELEMENT_COLOR);
            } else {
                env.set(
                    theme::CONTAINER_BACKGROUND_COLOR,
                    LIST_ELEMENT_COLOR_SELECTED,
                );
            }
        },
        Container::new(
            Padding::new(
                (25., 15., 15., 15.),
                Flex::row()
                    .with_child(Icon::new().lens(SearchResult::icon_path), 1.)
                    .with_child(
                        Flex::column()
                            .with_child(
                                Label::new(|item: &SearchResult, _env: &_| item.name.clone())
                                    .color(LABEL_MAIN_COLOR)
                                    .text_align(UnitPoint::LEFT),
                                1.0,
                            )
                            .with_child(
                                Label::new(|item: &SearchResult, _env: &_| {
                                    item.description.clone()
                                })
                                .color(LABEL_SECONDARY_COLOR)
                                .text_align(UnitPoint::LEFT),
                                1.0,
                            ),
                        8.,
                    ),
            )
            .fix_height(75.),
        ),
    )
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(AutoTextBox::new().lens(AppState::input_text), 1.)
        .with_child(List::new(list_element).lens(AppState::search_results), 3.)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((550., 320.00))
        .title(LocalizedString::new("launcherrr").with_placeholder(String::from("launcherrr")));
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
            env.set(theme::TEXT_SIZE_LARGE, 30.);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 2.);
            env.set(theme::WINDOW_BACKGROUND_COLOR, BG_COLOR);
            env.set(theme::LABEL_COLOR, Color::rgb8(0xf2, 0xf2, 0xf2));
            env.set(theme::LABEL_SECONDARY_COLOR, Color::rgb8(0xa2, 0xa2, 0xa2));
            env.set(theme::BACKGROUND_LIGHT, Color::rgb8(0x39, 0x3d, 0x40));
        })
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
