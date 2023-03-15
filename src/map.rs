use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let i = map_idx(x, y);
                match self.tiles[i] {
                    TileType::Floor => {
                        ctx.set(x, y, PURPLE, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, WHITE, to_cp437('#'));
                    }
                }
            }
        }
    }
}

// 2D Vector Row First Encoding
// x = index % width
// y = index / width rounded down

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
