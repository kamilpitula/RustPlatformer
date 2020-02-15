use piston::input::UpdateArgs;

pub trait Game_Object {
    fn update(&mut self, args: &UpdateArgs);
}