use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    winit::VirtualKeyCode,
};

use crate::fl_max::{Maxwell, AREA_HEIGHT, MAX_HEIGHT, Pipe, MAX_WIDTH, AREA_WIDTH, PIPE_WIDTH};

const BOUNCE_TIME: f32 = 30.0;
const BOUNCE_DISTANCE: f32 = 37.5;

#[derive(SystemDesc)]
pub struct BounceSystem {
    pub key_was_pressed: bool,
    pub bounce_on: bool,
    pub bounce_ticker: u8,
}

impl BounceSystem {
    pub fn new() -> Self {
        BounceSystem {
            key_was_pressed: false,
            bounce_on: false,
            bounce_ticker: 0,
        }
    }
}

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Maxwell>,
        ReadStorage<'s, Pipe>,
        Read<'s, InputHandler<StringBindings>>,
    );
    fn run(&mut self, (mut transforms, max, pipe, input): Self::SystemData) {
        let key_down_now = input.key_is_down(VirtualKeyCode::Space);
        if key_down_now && !self.key_was_pressed {
            //println!("Space!!!!!!!!!!!!!!!!!!!!!");
            /*for (m, t) in (&max, &mut transforms).join() {
                t.prepend_translation_y(15.0);
            }*/
            self.key_was_pressed = true;
            self.bounce_on = true;
            self.bounce_ticker = 0;
        }
        if !key_down_now {
            self.key_was_pressed = false;
        }
        if self.bounce_on == true {
            self.bounce_ticker += 2;
            if self.bounce_ticker >= BOUNCE_DISTANCE as u8 {
                self.bounce_ticker = 0;
                self.bounce_on = false;
            }
        }
        let mut max_y_pos: f32 = AREA_HEIGHT / 2.0;
        for (_m, t) in (&max, &mut transforms).join() {
            if self.bounce_on == false {
                let max_y = t.translation().y;
                t.set_translation_y(
                    (max_y - 0.4).max(MAX_HEIGHT * 0.2),
                    //.max(0.0),
                );
            } else {
                if t.translation()[1] < AREA_HEIGHT - (MAX_HEIGHT * 0.4) {
                    t.prepend_translation_y(0.55 * (BOUNCE_DISTANCE / BOUNCE_TIME));
                }
                if self.bounce_ticker <= (BOUNCE_DISTANCE as u8) / 2 {
                    t.set_rotation_z_axis(self.bounce_ticker as f32 / 95.0);
                } else {
                    t.set_rotation_z_axis((BOUNCE_DISTANCE - self.bounce_ticker as f32) / 95.0);
                }
            } 
            //if (t.translation().y + MAX_WIDTH / 2)
            max_y_pos = t.translation().y;
        }  
        for (_p, t) in (&pipe, &transforms).join() {
            if (AREA_WIDTH * 0.275) + (MAX_WIDTH / 2.0) - 2.0 >= t.translation().x - (PIPE_WIDTH / 2.0) 
                && ((AREA_WIDTH * 0.275) - (MAX_WIDTH / 2.0)) + 2.0 <= t.translation().x + (PIPE_WIDTH / 2.0) {
                if(max_y_pos - (MAX_HEIGHT / 2.0)) <= t.translation().y - (AREA_HEIGHT / 7.0)
                    ||(max_y_pos + (MAX_HEIGHT / 2.0)) >= t.translation().y + (AREA_HEIGHT / 7.0){
                    println!("Collision!!");
                }
            }
        }
    }
}
