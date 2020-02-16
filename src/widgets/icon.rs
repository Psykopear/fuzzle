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
    image_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl<T: Data> Icon<T> {
    pub fn new(image_path: impl Into<ImagePath<T>>) -> Self {
        let image_path = image_path.into();
        Self {
            image_path,
            image_data: vec![],
            width: 0,
            height: 0,
        }
    }
}

impl<T: Data> Widget<T> for Icon<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                let image_path = match &self.image_path {
                    ImagePath::Specific(path) => path.clone(),
                    ImagePath::Dynamic(closure) => {
                        let res: String = closure(data, env);
                        res.clone()
                    }
                };

                let im = image::open(&image_path).unwrap();
                let (width, height) = im.dimensions();
                self.width = width as usize;
                self.height = height as usize;
                if let Some(buffer) = im.as_rgba8() {
                    self.image_data = buffer.to_vec();
                };
            }
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

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
        let size = paint_ctx.size();
        let image = paint_ctx
            .make_image(
                self.width,
                self.height,
                &self.image_data.as_slice(),
                druid::piet::ImageFormat::RgbaSeparate,
            )
            .expect("Can't make image");
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ZERO, (self.width as f64, self.height as f64)),
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
