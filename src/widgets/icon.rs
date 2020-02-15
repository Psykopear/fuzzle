use druid::piet::{ImageFormat, InterpolationMode};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Rect, RenderContext, Size, UpdateCtx, Widget,
};

pub struct Icon {
    image_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Icon {
    pub fn new(image_data: Vec<u8>) -> Self {
        let decoder = png::Decoder::new(image_data.as_slice());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        let image_data: Vec<u8> = buf.into();
        Icon {
            image_data,
            width: info.width as usize,
            height: info.height as usize,
        }
    }
}

impl Widget<String> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

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
        let size = paint_ctx.size();
        let image = paint_ctx
            .make_image(
                self.width,
                self.height,
                &self.image_data,
                ImageFormat::RgbaSeparate,
            )
            .unwrap();
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ZERO, size),
            InterpolationMode::Bilinear,
        );
    }
}
