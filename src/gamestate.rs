use piston::input::{RenderArgs, UpdateArgs, Button};
use opengl_graphics::GlyphCache;
use super::states::State;
use super::gamedata::GameData;
use opengl_graphics::GlGraphics;
use graphics::Context;

pub trait GameState {
        fn render(&mut self, ctx: &Context, gl: &mut GlGraphics, glyphs: &mut GlyphCache);
        fn update(&mut self, args: &UpdateArgs) -> State<GameData>;
        fn key_press(&mut self, args: &Button);
        fn key_release(&mut self, args: &Button);
}