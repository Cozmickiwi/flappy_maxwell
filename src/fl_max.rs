extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use rand::Rng;

pub struct FlappyMaxwell;

impl SimpleState for FlappyMaxwell {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_maxwell_sprite(world);
        let sprite_sheet_handle_pipe = load_pipe_sprite(world);
        world.register::<Maxwell>();
        initialise_maxwell(world, sprite_sheet_handle);
        world.register::<Pipe>();
        initialise_pipe(world, sprite_sheet_handle_pipe);
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

#[derive(PartialEq, Eq)]
pub enum Order {
    First,
    Second,
}

pub struct Pipe {
    pub order: Order,
    pub width: f32,
    pub height: f32,
}

impl Pipe {
    fn new(order: Order) -> Pipe {
        Pipe {
            order,
            width: MAX_WIDTH,
            height: MAX_HEIGHT,
        }
    }
}

impl Component for Pipe {
    type Storage = DenseVecStorage<Self>;
}

pub const PIPE_WIDTH: f32 = 17.0;
pub const PIPE_HEIGHT: f32 = 170.0;

fn initialise_pipe(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut pipe_transform = Transform::default();
    let mut pipe_transform2 = Transform::default();
    let mut rng = rand::thread_rng();
    let random_num: i32 = rng.gen_range(1..=44);
    let ran_y = (random_num - 22) as f32 + 50.0;
    let random_num2: i32 = rng.gen_range(1..=44);
    let ran_y2 = (random_num2 - 22) as f32 + 50.0;
    pipe_transform.set_translation_xyz(AREA_WIDTH + PIPE_WIDTH, ran_y, 0.0);
    pipe_transform2.set_translation_xyz(
        AREA_WIDTH + PIPE_WIDTH * 2.0 + (AREA_WIDTH / 2.0),
        ran_y2,
        0.0,
    );
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Pipe::new(Order::First))
        .with(pipe_transform)
        .build();
    world
        .create_entity()
        .with(sprite_render)
        .with(Pipe::new(Order::Second))
        .with(pipe_transform2)
        .build();
}

fn load_pipe_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "pipe/pipe2.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "pipe/pipe.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
