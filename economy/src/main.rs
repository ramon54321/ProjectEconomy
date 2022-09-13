use economy::graphics::render;
use economy::simulation::simulate;
use economy::RenderableState;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<RenderableState>();

    // Simulation
    let simulation_thread = thread::spawn(move || simulate(tx));

    // Graphics
    render(rx);
}
