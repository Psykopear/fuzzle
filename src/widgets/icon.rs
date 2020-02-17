use druid::piet::InterpolationMode;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};
use image::GenericImageView;

pub enum ImagePath<T> {
    Specific(String),
    Dynamic(Box<dyn Fn(&T, &Env) -> String>),
}

pub struct Icon<T> {
    image_path: ImagePath<T>,
}

impl<T: Data> Icon<T> {
    pub fn new(image_path: impl Into<ImagePath<T>>) -> Self {
        let image_path = image_path.into();
        Self { image_path }
    }
}

impl<T: Data> Widget<T> for Icon<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        // TODO: This should be done only once, not every paint
        let image_path = match &self.image_path {
            ImagePath::Specific(path) => path.clone(),
            ImagePath::Dynamic(closure) => {
                let res: String = closure(data, env);
                res.clone()
            }
        };

        if let Ok(im) = image::open(&image_path) {
            let (width, height) = im.dimensions();
            if let Some(buffer) = im.as_rgba8() {
                let image = paint_ctx
                    .make_image(
                        width as usize,
                        height as usize,
                        &buffer,
                        druid::piet::ImageFormat::RgbaSeparate,
                    )
                    .expect("Can't make image");
                paint_ctx.draw_image(
                    &image,
                    Rect::from_origin_size(Point::ZERO, (width as f64, height as f64)),
                    InterpolationMode::Bilinear,
                );
            };
        };
    }
}

impl<T> From<String> for ImagePath<T> {
    fn from(src: String) -> ImagePath<T> {
        ImagePath::Specific(src)
    }
}

impl<T> From<&str> for ImagePath<T> {
    fn from(src: &str) -> ImagePath<T> {
        ImagePath::Specific(src.to_string())
    }
}

impl<T, F: Fn(&T, &Env) -> String + 'static> From<F> for ImagePath<T> {
    fn from(src: F) -> ImagePath<T> {
        ImagePath::Dynamic(Box::new(src))
    }
}
