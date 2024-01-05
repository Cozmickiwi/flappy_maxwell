extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Source},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiBundle, UiText, UiTransform},
};

use rand::Rng;

pub struct FlappyMaxwell;

pub struct GameOver;

impl SimpleState for FlappyMaxwell {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_maxwell_sprite(world);
        let sprite_sheet_handle_pipe = load_pipe_sprite(world);
        let sprite_sheet_handle_background = load_background_texture(world);
        world.register::<Maxwell>();
        initialise_maxwell(world, sprite_sheet_handle);
        world.register::<Pipe>();
        initialise_pipe(world, sprite_sheet_handle_pipe);
        initialise_score(world);
        world.register::<Background>();
        initialise_background(world, sprite_sheet_handle_background);
        initialise_camera(world);
    }
}
/*
impl SimpleState for GameOver {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
    }
}
*/
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
    pipe_transform.set_translation_xyz(AREA_WIDTH + PIPE_WIDTH, ran_y, -0.01);
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

#[derive(Default, Debug)]
pub struct Score {
    pub score: i32,
}

impl Component for Score {
    type Storage = DenseVecStorage<Self>;
}

pub struct ScoreText {
    pub score_text: Entity,
}

fn initialise_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/flappy-font.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let ui_transform = UiTransform::new(
        "ui_text".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.0,
        -50.0,
        0.0,
        200.0,
        200.0,
    );
    let score_text = world
        .create_entity()
        .with(ui_transform)
        .with(UiText::new(
            font,
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            90.0,
            amethyst::ui::LineMode::Wrap,
            Anchor::TopMiddle,
        ))
        .build();
    world.insert(ScoreText { score_text });
}

const BCG_WIDTH: f32 = 1300.0;
const BGC_HEIGHT: f32 = 600.0;

pub struct Background {
    pub width: f32,
    pub height: f32,
}

impl Background {
    fn new() -> Background {
        Background {
            width: BCG_WIDTH,
            height: BGC_HEIGHT,
        }
    }
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}
fn initialise_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
    let mut background_transform = Transform::default();
    background_transform.set_translation_xyz(AREA_WIDTH / 2.0, AREA_HEIGHT / 2.0, -0.11);
    world
        .create_entity()
        .with(sprite_render)
        .with(background_transform)
        .with(Background::new())
        .build();

}

fn load_background_texture(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "background/8bitsky.jpg",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "background/8bitsky.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
