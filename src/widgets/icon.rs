use druid::piet::InterpolationMode;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Rect, RenderContext, Size, UpdateCtx, Widget,
};
use image::GenericImageView;

pub struct Icon {
    data: Option<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Icon {
    pub fn new() -> Self {
        Self {
            data: None,
            height: 0,
            width: 0,
        }
    }

    fn resolve_icon(&mut self, data: &String) {
        if let Ok(im) = image::open(data) {
            if let Some(buffer) = im.as_rgba8() {
                let (width, height) = im.dimensions();
                self.data = Some(buffer.to_vec());
                self.width = width as usize;
                self.height = height as usize;
                return;
            };
        };
        // If we didn't return, set a default image
        let im = image::load_from_memory(include_bytes!("../assets/default.png")).unwrap();
        let (width, height) = im.dimensions();
        self.data = Some(im.as_rgba8().unwrap().to_vec());
        self.width = width as usize;
        self.height = height as usize;
    }
}

impl Widget<String> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut String, _env: &Env) {
        match event {
            Event::WindowConnected => self.resolve_icon(data),
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &String, data: &String, _env: &Env) {
        if !old_data.eq(data) {
            self.resolve_icon(data);
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        if let Some(data) = &self.data {
            let image = match paint_ctx.make_image(
                self.width,
                self.height,
                &data,
                druid::piet::ImageFormat::RgbaSeparate,
            ) {
                Ok(image) => image,
                Err(_) => return,
            };

            paint_ctx.draw_image(
                &image,
                Rect::from_origin_size(Point::ZERO, (self.width as f64, self.height as f64)),
                InterpolationMode::Bilinear,
            );
        }
    }
}
