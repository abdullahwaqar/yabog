use macroquad::prelude::*;

const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;
const PLAYER_SPEED: f32 = 700f32;
const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 40f32]);

fn window_conf() -> Conf {
    Conf {
        window_title: "YAOG".to_owned(),
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKBLUE);
    }

    pub fn tick(&mut self, delta: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        // * Move the player
        self.rect.x += x_move * delta * PLAYER_SPEED;
        // println!("Current x_pos: {:?}", self.rect.x);

        // * Check wall collision
        if self.rect.x < 0f32 {
            println!("Colliding with left");
            self.rect.x = 0f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            println!("Colliding with right");
            self.rect.x = screen_width() - self.rect.w;
        }
    }
}

struct Block {
    rect: Rect,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: BLOCK_SIZE.x,
                h: BLOCK_SIZE.y,
            },
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKBLUE);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();

    let (width, height) = (6, 4);
    let board_start_pos = vec2(
        (screen_width() - (BLOCK_SIZE.x * width as f32)) * 0.5f32,
        50f32,
    );

    for i in 0..width * height {
        let block_x = (i % width) as f32 * BLOCK_SIZE.x;
        let block_y = (i / width) as f32 * BLOCK_SIZE.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    loop {
        player.tick(get_frame_time());

        clear_background(WHITE);

        player.draw();

        next_frame().await
    }
}
