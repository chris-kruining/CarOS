// use anyhow::Result;
// use smithay::{
//     backend::{
//         renderer::{
//             element::{texture::TextureRenderElement, Element, RenderElement},
//             gles::GlesTexture,
//             glow::GlowRenderer,
//             Frame, Renderer,
//         },
//         winit,
//     },
//     input::{
//         keyboard::{FilterResult, XkbConfig},
//         pointer::{AxisFrame, ButtonEvent, MotionEvent},
//         SeatHandler, SeatState,
//     },
//     utils::{Rectangle, Transform, SERIAL_COUNTER},
// };
// use smithay_egui::EguiState;
//
//
// impl SeatHandler for State {
//     type KeyboardFocus = EguiState;
//     type PointerFocus = EguiState;
//     fn seat_state(&mut self) -> &mut SeatState<Self> {
//         &mut self.seat_state
//     }
// }
//
// struct State {
//     seat_state: SeatState<State>,
// }
//
// pub fn integrate() -> Result<()> {
//     // create a winit-backend
//     let (mut backend, mut input) = winit::init::<GlowRenderer>().map_err(|_| anyhow::anyhow!("Winit failed to start"))?;
//
//     let egui = EguiState::new(Rectangle::from_loc_and_size(
//         (0, 0),
//         backend.window_size().physical_size.to_logical(1),
//     ));
//
//     Ok(())
// }