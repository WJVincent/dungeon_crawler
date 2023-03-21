pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

// empty struct serves as a tag, indicating that this entity is a player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
