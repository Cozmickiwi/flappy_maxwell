use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct FlappyMaxwell;

impl SimpleState for FlappyMaxwell {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    Ok(())
}
