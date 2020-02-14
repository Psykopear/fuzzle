use druid::piet::{ImageFormat, InterpolationMode};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Rect, RenderContext, Size, UpdateCtx, Widget,
};

pub struct Icon {
    image_data: Vec<u8>
}

impl Icon {
    pub fn new(image_data: Vec<u8>) -> Self {
        Icon { image_data }
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
        let decoder = png::Decoder::new(self.image_data.as_slice());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        let image_data: Vec<u8> = buf.into();
        let image = paint_ctx
            .make_image(
                info.width as usize,
                info.height as usize,
                &image_data,
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
