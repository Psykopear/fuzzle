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
    image_path_resolved: Option<String>,
}

impl<T: Data> Icon<T> {
    pub fn new(image_path: impl Into<ImagePath<T>>) -> Self {
        let image_path = image_path.into();
        Self {
            image_path,
            image_path_resolved: None,
        }
    }

    fn resolve_image_path(&mut self, data: &T, env: &Env) {
        self.image_path_resolved = match &self.image_path {
            ImagePath::Specific(path) => Some(path.clone()),
            ImagePath::Dynamic(closure) => Some(closure(data, env).clone()),
        };
    }
}

impl<T: Data + PartialEq> Widget<T> for Icon<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => self.resolve_image_path(data, env),
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if old_data != data {
            self.resolve_image_path(data, env);
            ctx.request_paint();
        }
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

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        let image_path = match self.image_path_resolved {
            Some(ref i) => i,
            None => return,
        };

        let im = match image::open(&image_path) {
            Ok(im) => im,
            Err(_) => return,
        };

        let buffer = match im.as_rgba8() {
            Some(b) => b,
            None => return,
        };

        let (width, height) = im.dimensions();

        let image = match paint_ctx.make_image(
            width as usize,
            height as usize,
            &buffer,
            druid::piet::ImageFormat::RgbaSeparate,
        ) {
            Ok(image) => image,
            Err(_) => return,
        };

        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ZERO, (width as f64, height as f64)),
            InterpolationMode::Bilinear,
        );
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
