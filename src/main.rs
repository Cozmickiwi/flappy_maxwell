use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, pipeline,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod fl_max;
mod systems;

use crate::fl_max::FlappyMaxwell;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            /*systems::BounceSystem {
                key_was_pressed: false,
                bounce_on: false,
                bounce_ticker: 0,
            },*/
            systems::BounceSystem::new(),
            "bounce_system",
            &["input_system"],
        )
        .with(systems::PipeSystem, "pipe_movement", &["input_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.16471, 0.32157, 0.74510, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?;
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, FlappyMaxwell, game_data)?;
    game.run();
    Ok(())
}
