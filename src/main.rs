use macroquad::prelude::*;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

use macroquad_tiled as tiled;

pub const MOVE_SPEED: f32 = 5.;
pub const WORLD_SIZE: f32 = 500.;

enum Direction {
    Up, Down, Left, Right
}

struct MarioSprites {
    up: Texture2D,
    down: Texture2D,
    left: Texture2D,
    right: Texture2D
}

struct Mario {
    pub x: f32,
    pub y: f32,
    direction: Direction,
    sprites: MarioSprites
}

impl Mario {    pub const MOVE_SPEED: f32 = 10.;
    async fn new(x: f32, y: f32) -> Self {
        let down = load_texture("assets/mario_down.png").await.expect("meh");
        let up = load_texture("assets/mario_up.png").await.expect("meh");
        let left = load_texture("assets/mario_left.png").await.expect("meh");
        let right = load_texture("assets/mario_right.png").await.expect("meh");

        Mario { x, y, direction: Direction::Down, sprites: MarioSprites{ up, down, left, right} }
    }

    pub fn record_move(&mut self) {
        if is_key_down(KeyCode::Right) && self.x < WORLD_SIZE {
            self.x += MOVE_SPEED;
            self.direction = Direction::Right;
        }

        if is_key_down(KeyCode::Left) && self.x > 0. {
            self.x -= MOVE_SPEED;
            self.direction = Direction::Left;
        }
        
        if is_key_down(KeyCode::Up) && self.y > 0. {
            self.y -= MOVE_SPEED;
            self.direction = Direction::Up;
        }
        
        if is_key_down(KeyCode::Down) && self.y < WORLD_SIZE {
            self.y += MOVE_SPEED;
            self.direction = Direction::Down;
        }
    }

    pub fn get_sprite(&self) -> &Texture2D {
        match self.direction {
            Direction::Up => &self.sprites.up,
            Direction::Down => &self.sprites.down,
            Direction::Left => &self.sprites.left,
            Direction::Right => &self.sprites.right,
        }
    }
}


#[macroquad::main("ZoKoBan")]
async fn main() {
    let wall = load_texture("assets/wall.png").await.expect("meh");
    let boxes = load_texture("assets/box.png").await.expect("meh1");
    let goal = load_texture("assets/goal.png").await.expect("meh2");

    // let tiled_map_json = load_string("assets/level1.json").await.expect("meh4");

    // let tiled_map = tiled::load_map(
    //     &tiled_map_json,
    //     &[("wall.png", wall), ("box.png", boxes), ("goal.png", goal)],
    //     &[],
    // ).expect("more meh");

    let mut player = Mario::new(1., 1.).await;

    loop {
        clear_background(RED);

        draw_texture_ex(&player.get_sprite(), player.x, player.y, WHITE, DrawTextureParams { source: Some(Rect::new(0.0, 0.0, 76., 66.)), ..Default::default()});

        player.record_move();

        next_frame().await
    }

}