use self::layers::{basic_info::render_basic_info, market_listings::render_market_listings};
use crate::RenderableState;
use femtovg::FontId;
use powder::{Powder, RenderLayerFn};
use std::sync::mpsc::Receiver;

mod layers;

pub struct PowderState {
    renderable_state: RenderableState,
    font: Option<FontId>,
}
impl Default for PowderState {
    fn default() -> Self {
        Self {
            renderable_state: Default::default(),
            font: None,
        }
    }
}

pub fn render(rx: Receiver<RenderableState>) {
    // Define layers for powder renderer
    let layers: Vec<RenderLayerFn<PowderState>> = vec![
        Box::new(render_basic_info),
        Box::new(render_market_listings),
    ];

    // Setup powder instance with initial state
    let mut powder = Powder::new(PowderState::default()).expect("Could not start powder");

    // Load after-init assets
    powder.state.font = Some(powder.load_font("assets/Roboto-Regular.ttf"));

    // Push thread receiver layer
    powder.push(Box::new(move |_canvas, _meta, state| {
        match rx.try_recv() {
            Ok(new_renderable_state) => state.renderable_state = new_renderable_state,
            _ => (),
        };
    }));

    // Push custom layers
    for layer in layers {
        powder.push(layer);
    }

    // Start graphics
    powder.start();
}
