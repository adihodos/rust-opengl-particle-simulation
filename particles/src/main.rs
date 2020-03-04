#![allow(dead_code)]

use sys::input::*;
use sys::SimpleWindow;

mod particles;

fn main() -> std::result::Result<(), String> {
    let mut app_window = SimpleWindow::new()?;

    let world_size = dbg!(app_window.size());
    let particle_sim = particles::ParticlesSim::new(world_size.0, world_size.1)?;
    app_window.message_loop(Box::new(move |e: &Event| particle_sim.main_loop(e)));

    Ok(())
}
