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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if Map::in_bounds(Point::new(x, y)) {
                    let idx = find_idx(x, y);
                    match self.tiles.get(idx) {
                        Some(TileType::Floor) => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                PURPLE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        Some(TileType::Wall) => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                SADDLE_BROWN,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn in_bounds(point: Point) -> bool {
        let in_x = point.x >= 0 && point.x < SCREEN_WIDTH;
        let in_y = point.y >= 0 && point.y < SCREEN_HEIGHT;

        in_x && in_y
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        Map::in_bounds(point) && self.tiles[find_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(point: Point) -> Option<usize> {
        if Map::in_bounds(point) {
            Some(find_idx(point.x, point.y))
        } else {
            None
        }
    }
}

// 2D Vector Row First Encoding
// x = index % width
// y = index / width rounded down

pub fn find_idx(x: i32, y: i32) -> usize {
    usize::try_from((y * SCREEN_WIDTH) + x).unwrap()
}
