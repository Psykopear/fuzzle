// use druid::piet::{ImageFormat, InterpolationMode};
// use druid::{
//     BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
//     Point, Rect, RenderContext, Size, UpdateCtx, Widget,
// };

// #[derive(Clone, Data)]
// pub struct Icon {
//     image_data: &[u8],
//     width: usize,
//     height: usize,
// }



// impl<T: Data> Icon<T> {
//     pub fn new(image_data: impl Into<&[u8]>) -> Self {
//         let image_data = image_data.into();
//         let decoder = png::Decoder::new(image_data.as_slice());
//         let (info, mut reader) = decoder.read_info().unwrap();
//         let mut buf = vec![0; info.buffer_size()];
//         reader.next_frame(&mut buf).unwrap();
//         let image_data: T = buf.into();
//         Self {
//             image_data,
//             width: info.width as usize,
//             height: info.height as usize,
//         }
//     }
// }

// impl<T: Data> Widget<T> for Icon<T> {
//     fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

//     fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

//     fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

//     fn layout(
//         &mut self,
//         _layout_ctx: &mut LayoutCtx,
//         bc: &BoxConstraints,
//         _data: &T,
//         _env: &Env,
//     ) -> Size {
//         bc.max()
//     }

//     fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &T, _env: &Env) {
//         let size = paint_ctx.size();
//         let image = paint_ctx
//             .make_image(
//                 self.width,
//                 self.height,
//                 &self.image_data,
//                 ImageFormat::RgbaSeparate,
//             )
//             .unwrap();
//         paint_ctx.draw_image(
//             &image,
//             Rect::from_origin_size(Point::ZERO, size),
//             InterpolationMode::Bilinear,
//         );
//     }
// }
