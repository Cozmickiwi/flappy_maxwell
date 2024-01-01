use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    winit::VirtualKeyCode,
};

use rand::Rng;

use crate::fl_max::{Pipe, AREA_HEIGHT, AREA_WIDTH, PIPE_WIDTH};

#[derive(SystemDesc)]
pub struct PipeSystem;

impl<'s> System<'s> for PipeSystem {
    type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Pipe>,
    );

    fn run(&mut self, (mut transforms, pipe): Self::SystemData) {
       for (p, t) in (&pipe, &mut transforms).join() {
            t.prepend_translation_x(-0.1);
            if t.translation()[0] <= -PIPE_WIDTH {
                t.set_translation_x(AREA_WIDTH + PIPE_WIDTH);
            }
        } 
    }
}
