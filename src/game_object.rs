use piston::input::{RenderArgs, UpdateArgs, Button};

pub trait Game_Object {
    fn update(&mut self, args: &UpdateArgs);
    fn key_press(&mut self, args: &Button);
}