extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct FlappyMaxwell;

impl SimpleState for FlappyMaxwell {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_maxwell_sprite(world);
        world.register::<Maxwell>();
        initialise_maxwell(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub const AREA_HEIGHT: f32 = 100.0;
pub const AREA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(AREA_WIDTH * 0.5, AREA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(AREA_WIDTH, AREA_HEIGHT))
        .with(transform)
        .build();
}

pub const MAX_HEIGHT: f32 = 12.0;
pub const MAX_WIDTH: f32 = 18.0;

pub struct Maxwell {
    pub width: f32,
    pub height: f32,
}

impl Maxwell {
    fn new() -> Maxwell {
        Maxwell {
            width: MAX_WIDTH,
            height: MAX_HEIGHT,
        }
    }
}

impl Component for Maxwell {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_maxwell(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut max_transform = Transform::default();
    max_transform.set_translation_xyz(AREA_WIDTH * 0.275, 50.0, 0.0);
    world
        .create_entity()
        .with(sprite_render)
        .with(Maxwell::new())
        .with(max_transform)
        .build();
}

fn load_maxwell_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/maxwell.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/maxwell.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
