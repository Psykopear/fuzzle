use druid::kurbo::{Point, Rect, Size};
use druid::piet::{
    CairoTextLayout, Color, FontBuilder, InterpolationMode, RenderContext, Text, TextLayout,
    TextLayoutBuilder, UnitPoint,
};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget,
};
use image::GenericImageView;

use crate::SearchResult;

const MAIN_COLOR: Color = Color::rgb8(0xc2, 0xc2, 0xc2);
const SECONDARY_COLOR: Color = Color::rgb8(0x92, 0x92, 0x92);
const PADDING: f64 = 20.;

/// A list element that displays a searchresult
pub struct ListElement {
    name: Option<CairoTextLayout>,
    name_font_size: f64,
    description: Option<CairoTextLayout>,
    description_font_size: f64,
    icon_data: Option<Vec<u8>>,
    icon_width: usize,
    icon_height: usize,
    selected: bool,
}

impl ListElement {
    pub fn new() -> Self {
        Self {
            name: None,
            name_font_size: 20.,
            description: None,
            description_font_size: 17.,
            icon_data: None,
            icon_height: 0,
            icon_width: 0,
            selected: false,
        }
    }

    fn resolve_icon(&mut self, data: &SearchResult) {
        if let Some(icon_path) = &data.icon_path {
            if let Ok(im) = image::open(icon_path) {
                if let Some(buffer) = im.as_rgba8() {
                    let (width, height) = im.dimensions();
                    self.icon_data = Some(buffer.to_vec());
                    self.icon_width = width as usize;
                    self.icon_height = height as usize;
                    return;
                };
            };
        }
        // If we didn't return, set a default image
        let im = image::load_from_memory(include_bytes!("../assets/default.png")).unwrap();
        let (width, height) = im.dimensions();
        self.icon_data = Some(im.as_rgba8().unwrap().to_vec());
        self.icon_width = width as usize;
        self.icon_height = height as usize;
    }

    fn resolve(&mut self, ctx: &mut PaintCtx, data: &SearchResult) {
        self.resolve_icon(data);
        let font_name = "sans-serif";

        let name_font = ctx
            .text()
            .new_font_by_name(font_name, self.name_font_size)
            .build()
            .unwrap();

        let description_font = ctx
            .text()
            .new_font_by_name(font_name, self.description_font_size)
            .build()
            .unwrap();
        self.name = Some(
            ctx.text()
                .new_text_layout(&name_font, &data.name)
                .build()
                .unwrap(),
        );
        self.description = Some(
            ctx.text()
                .new_text_layout(&description_font, &data.description)
                .build()
                .unwrap(),
        );
        self.selected = data.selected;
    }
}

impl Widget<SearchResult> for ListElement {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut SearchResult, _env: &Env) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &SearchResult,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &SearchResult,
        _data: &SearchResult,
        _env: &Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &SearchResult,
        _env: &Env,
    ) -> Size {
        bc.debug_check("ListElement");
        if let Some(name) = &self.name {
            if let Some(description) = &self.description {
                bc.constrain(Size::new(name.width().max(description.width()), 75.))
            } else {
                bc.constrain(Size::new(name.width(), 75.))
            }
        } else {
            bc.constrain(Size::new(0., 75.))
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, search_result: &SearchResult, _env: &Env) {
        self.resolve(paint_ctx, search_result);
        if let Some(data) = &self.icon_data {
            let image = match paint_ctx.make_image(
                self.icon_width,
                self.icon_height,
                &data,
                druid::piet::ImageFormat::RgbaSeparate,
            ) {
                Ok(image) => image,
                Err(_) => return,
            };

            paint_ctx.draw_image(
                &image,
                Rect::from_origin_size(
                    Point::from((PADDING + 8., 14.)),
                    (self.icon_width as f64, self.icon_height as f64),
                ),
                InterpolationMode::Bilinear,
            );
        }

        if self.selected {
            let width = paint_ctx.size().width;
            let height = paint_ctx.size().height;
            paint_ctx.fill(
                Rect::from_origin_size(Point::ORIGIN, Size::new(width, height)),
                &Color::rgba8(0xff, 0xff, 0xff, 0x22),
            )
        }
        if let Some(name) = &self.name {
            let name_origin = UnitPoint::LEFT.resolve(Rect::from_origin_size(
                Point::from((PADDING + 64., 16.)),
                Size::new(
                    (paint_ctx.size().width - name.width()).max(0.0),
                    paint_ctx.size().height / 2.,
                ),
            ));

            paint_ctx.draw_text(&name, name_origin, &MAIN_COLOR);
        }
        if let Some(description) = &self.description {
            let description_origin = UnitPoint::LEFT.resolve(Rect::from_origin_size(
                Point::from((PADDING + 64., paint_ctx.size().height / 2.)),
                Size::new(
                    (paint_ctx.size().width - description.width()).max(0.0),
                    paint_ctx.size().height / 2.,
                ),
            ));
            paint_ctx.draw_text(&description, description_origin, &SECONDARY_COLOR);
        }
    }
}
