use macroquad::prelude::*;

const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;
const PLAYER_SPEED: f32 = 700f32;
const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 40f32]);

fn window_conf() -> Conf {
    Conf {
        window_title: "YABOG".to_owned(),
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
            rect: Rect {
                x: screen_width() / 2f32 - PLAYER_SIZE.x * 0.5f32,
                y: screen_height() - 100f32,
                w: PLAYER_SIZE.x,
                h: PLAYER_SIZE.y,
            },
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

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
}
struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: BLOCK_SIZE.x,
                h: BLOCK_SIZE.y,
            },
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => RED,
                _ => ORANGE,
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    velocity: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: BALL_SIZE,
                h: BALL_SIZE,
            },
            velocity: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKBLUE);
    }

    pub fn tick(&mut self, delta: f32) {
        self.rect.x += self.velocity.x * delta * BALL_SPEED;
        self.rect.y += self.velocity.y * delta * BALL_SPEED;

        // * Collisions
        if self.rect.x < 0f32 {
            self.rect.x = 1f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = -1f32;
        }

        // * Celling collision
        if self.rect.y < 0f32 {
            self.velocity.y = 1f32;
        }
    }
}

/// aabb Collision with some sort of positional correction
fn collision_resolver(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // * Bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // * Bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    return true;
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 1f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(
            board_start_pos + vec2(block_x, block_y),
            BlockType::Regular,
        ));
    }
    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();

    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    )));

    init_blocks(&mut blocks);

    loop {
        // * MORE BALLS LOL
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2(
                screen_width() * 0.5f32,
                screen_height() * 0.5f32,
            )));
        }

        player.tick(get_frame_time());

        balls
            .iter_mut()
            .for_each(|ball| ball.tick(get_frame_time()));

        for ball in balls.iter_mut() {
            collision_resolver(&mut ball.rect, &mut ball.velocity, &player.rect);
            // Block to ball collision
            for block in blocks.iter_mut() {
                if collision_resolver(&mut ball.rect, &mut ball.velocity, &block.rect) {
                    block.lives -= 1;
                }
            }
        }

        // * Remove all the dead blocks
        blocks.retain(|block| block.lives > 0);

        clear_background(WHITE);

        player.draw();

        blocks.iter().for_each(|block| block.draw());
        balls.iter().for_each(|ball| ball.draw());

        next_frame().await
    }
}
