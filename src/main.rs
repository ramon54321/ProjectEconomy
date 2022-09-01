use economy::graphics::render;
use economy::simulation::simulate;
use economy::State;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<State>();

    // Simulation
    let simulation_thread = thread::spawn(move || simulate(tx));

    // Graphics
    render(rx);
}
