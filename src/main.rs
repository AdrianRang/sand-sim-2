use std::{thread::sleep, time::Duration, usize};

use macroquad::prelude::*;
use macroquad::input;


const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const SCALE: f32 = 10.0;

struct Pixels {
    pixels: Vec<Color>
}

#[derive(Clone, Copy, PartialEq)]
struct Particle {
    x: usize,
    y: usize,
    color: Color,
    tpe: Type,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Type {
    LOOSE,
    SOLID,
    LIQUID
}

impl Pixels {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y * WIDTH + x] = color;
        }
    }
}

impl Particle {
    fn update(&mut self, particles: &Vec<Particle>)  {
        if contains_particle(particles, *self) {println!("Dupe, {:?}", self.tpe);}
        match self.tpe {
            Type::LOOSE => {
                if self.y == HEIGHT-1 { }
                else if !contains(particles, self.x, self.y + 1) {
                    self.y += 1;
                } else if self.x != WIDTH-1 && !contains(particles, self.x + 1, self.y + 1) {
                    self.x += 1;
                    self.y += 1;
                } else if self.x != 0 && !contains(particles, self.x - 1, self.y + 1) {
                    self.y += 1;
                    self.x -= 1;
                } else if particles.into_iter().any(|p| p.x == self.x && p.y == self.y + 1 && p.tpe == Type::LIQUID) && !particles.into_iter().any(|p| p.x == self.x && p.y == self.y + 1 && (p.tpe == Type::LOOSE || p.tpe == Type::SOLID)) {
                    let mut p = *particles.into_iter().find(|p| p.x == self.x && p.y == self.y + 1 && p.tpe == Type::LIQUID).unwrap();
                    p.y -= 1;
                    self.y += 1;
                } else if self.x != WIDTH-1 && particles.into_iter().any(|p| p.x == self.x + 1 && p.y == self.y + 1 && p.tpe == Type::LIQUID) && !particles.into_iter().any(|p| p.x == self.x + 1 && p.y == self.y + 1 && (p.tpe == Type::LOOSE || p.tpe == Type::SOLID)) {
                    let mut p = *particles.into_iter().find(|p| p.x == self.x + 1 && p.y == self.y + 1 && p.tpe == Type::LIQUID).unwrap();
                    p.y = self.y;
                    p.x = self.x;
                    self.y += 1;
                    self.x += 1;
                } else if self.x != 0 && particles.into_iter().any(|p| p.x == self.x - 1 && p.y == self.y + 1 && p.tpe == Type::LIQUID) && !particles.into_iter().any(|p| p.x == self.x - 1 && p.y == self.y + 1 && (p.tpe == Type::LOOSE || p.tpe == Type::SOLID)) {
                    let mut p = *particles.into_iter().find(|p| p.x == self.x - 1 && p.y == self.y + 1 && p.tpe == Type::LIQUID).unwrap();
                    p.y = self.y;
                    p.x = self.x;
                    self.y += 1;
                    self.x -= 1;
                }
            }
            Type::SOLID => { }
            Type::LIQUID => {
                if self.y == HEIGHT {}
                else if contains_particle(particles, *self) {
                    println!("UP");
                    self.y -= 1;
                } else if !contains(particles, self.x, self.y + 1) {
                    self.y += 1;
                } else if self.x != WIDTH-1 && !contains(particles, self.x + 1, self.y + 1) {
                    self.x += 1;
                    self.y += 1;
                } else if self.x != 0 && !contains(particles, self.x - 1, self.y + 1) {
                    self.y += 1;
                    self.x -= 1;
                } else if self.x != 0 && self.x != WIDTH-1 && contains(particles, self.x, self.y + 1) && contains(particles, self.x + 1, self.y) && !contains(particles, self.x - 1, self.y) {
                    self.x -= 1;
                } else if self.x != WIDTH-1 && self.x != 0 && contains(particles, self.x, self.y + 1) && contains(particles, self.x - 1, self.y) && !contains(particles, self.x + 1, self.y) {
                    self.x += 1;
                }
            }
        }
    }
}

fn contains(prtcls: &Vec<Particle>, x: usize, y: usize) -> bool {
    prtcls.into_iter().any(|p| p.x == x && p.y == y)
}

fn contains_particle(prtcls: &Vec<Particle>, prtcl: Particle) -> bool {
    prtcls.into_iter().any(|p| p.x == prtcl.x && p.y == prtcl.y && p != &prtcl)
}

#[macroquad::main("SandSim")]
async fn main() {
    request_new_screen_size(WIDTH as f32 * SCALE, HEIGHT as f32 *SCALE + 27.0);
    let mut image = Image::gen_image_color(
        WIDTH as u16,
        HEIGHT as u16,
        BLACK,
    );

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut pixels: Pixels;
    let mut particles: Vec<Particle> = Vec::new();

    loop {
        let mouse_pos = ((input::mouse_position().0 / SCALE) as usize, (input::mouse_position().1 / SCALE) as usize);

        if input::is_mouse_button_down(MouseButton::Left) {
            for y in -1..1 {
                for x in -1..1 {
                    if (mouse_pos.0 as i32 + x) < 0 {continue;}
                    if (mouse_pos.0 as i32 + x) >= WIDTH as i32 {continue;}
                    if (mouse_pos.1 as i32 + y) >= HEIGHT as i32 {continue;}
                    if (mouse_pos.1 as i32 + y) < 0 as i32 {continue;}
                    if !particles.clone().into_iter().any(|p| p.x == (mouse_pos.0 as i32 + x) as usize && p.y == (mouse_pos.1 as i32 + y) as usize) {
                        particles.push(Particle {x: (mouse_pos.0 as i32 + x) as usize, y: (mouse_pos.1 as i32 + y) as usize, color: if !input::is_key_down(KeyCode::LeftShift) {random_sand_col()} else {GRAY}, tpe: if !input::is_key_down(KeyCode::LeftShift) {Type::LOOSE} else {Type::SOLID} });
                    }
                }
            }
        }
        if input::is_key_down(KeyCode::W) {
            for y in -1..1 {
                for x in -1..1 {
                    if (mouse_pos.0 as i32 + x) < 0 {continue;}
                    if (mouse_pos.0 as i32 + x) >= WIDTH as i32 {continue;}
                    if (mouse_pos.1 as i32 + y) >= HEIGHT as i32 {continue;}
                    if (mouse_pos.1 as i32 + y) < 0 as i32 {continue;}
                    if !particles.clone().into_iter().any(|p| p.x == (mouse_pos.0 as i32 + x) as usize && p.y == (mouse_pos.1 as i32 + y) as usize) {
                        particles.push(Particle {x: (mouse_pos.0 as i32 + x) as usize, y: (mouse_pos.1 as i32 + y) as usize, color: Color::from_rgba(0, 0, rand::gen_range(100, 175), 200), tpe: Type::LIQUID});
                    }
                }
            }
        }

        if input::is_key_down(KeyCode::S) {
            particles.push(Particle { x: rand::gen_range(0, WIDTH), y: 0, color: random_sand_col(), tpe: Type::LOOSE });
        }

        if input::is_key_down(KeyCode::R) {
            let new_p = Particle { x: rand::gen_range(0, WIDTH), y: 0, color: Color::from_rgba(0, 0, rand::gen_range(100, 175), 200), tpe: Type::LIQUID };
            if !contains_particle(&particles, new_p) {particles.push(new_p);}
        }

        if input::is_mouse_button_down(MouseButton::Right) {
            for y in -5..5 {
                for x in -5..5 {
                    particles.retain(|p| !(p.x == (mouse_pos.0 as i32 + x) as usize && p.y == (mouse_pos.1 as i32 + y) as usize));
                }
            } 
        }

        pixels = Pixels { pixels: vec![BLACK; WIDTH*HEIGHT] };
        for i in 0..particles.len()  {
            pixels.set_pixel(particles.get(i).unwrap().x, particles.get(i).unwrap().y, particles.get(i).unwrap().color);
            
            let mut p = *particles.get(i).unwrap();
            p.update(&particles);
            let len = particles.len();
            particles.push(p);
            particles.swap(i, len);
            particles.pop();
        }

        // for i in 0..particles.len() {
        //     if particles.get(i) == None {break;}
        //     if contains_particle(&particles, *particles.get(i).unwrap()) {
        //         particles.remove(i);
        //     }
        // }

        // Draw lagic
        ///////////////////////////////////////////////////////////////////////

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                image.set_pixel(
                    x as u32,
                    y as u32,
                    pixels.pixels[y * WIDTH + x],
                );
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    WIDTH as f32 * SCALE,
                    HEIGHT as f32 * SCALE,
                )),
                ..Default::default()
            },
        );



        next_frame().await;
    }
}

fn random_sand_col() -> Color {
    Color::from_rgba(rand::gen_range(200, 200), rand::gen_range(144, 220), rand::gen_range(32, 75), 255)
}