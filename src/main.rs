extern crate nalgebra_glm as glm;
extern crate minifb;

use minifb::{Window, WindowOptions, Key};
use glm::Vec3;
use std::time::Duration;

mod framebuffer;
use crate::framebuffer::{Framebuffer, Color};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Lab 2",
        WIDTH * 8,
        HEIGHT * 8,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    add_glider(&mut grid, 1, 1);
    add_glider(&mut grid, 1, 70);
    add_glider(&mut grid, 30, 20);
    add_glider(&mut grid, 60, 1);
    add_glider(&mut grid, 70, 62);
    add_pulsar(&mut grid, 10, 10);
    add_pulsar(&mut grid, 70, 10);
    add_pulsar(&mut grid, 40, 40);
    add_pulsar(&mut grid, 70, 70);
    add_pulsar(&mut grid, 10, 70);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer, &grid);
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        grid = next_generation(&grid);

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] {
                framebuffer.set_current_color(Color(0xFFFFFF)); 
                framebuffer.point(x, y);
            } else {
                framebuffer.set_current_color(Color(0x000000)); 
                framebuffer.point(x, y);
            }
        }
    }
}

fn next_generation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive_neighbors = count_alive_neighbors(grid, x, y);
            if grid[y][x] {
                new_grid[y][x] = alive_neighbors == 2 || alive_neighbors == 3;
            } else {
                new_grid[y][x] = alive_neighbors == 3;
            }
        }
    }
    new_grid
}

fn count_alive_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in [-1, 0, 1].iter().cloned() {
        for dx in [-1, 0, 1].iter().cloned() {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < WIDTH && ny < HEIGHT && grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

fn add_glider(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let glider_pattern = vec![
        (1, 0),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];

    for &(dx, dy) in &glider_pattern {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        if nx < WIDTH && ny < HEIGHT {
            grid[ny][nx] = true;
        }
    }
}

// Function to add a pulsar (period 3 oscillator)
fn add_pulsar(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let pulsar_pattern = vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];

    for &(dx, dy) in &pulsar_pattern {
        let nx = x.wrapping_add(dx);
        let ny = y.wrapping_add(dy);
        if nx < WIDTH && ny < HEIGHT {
            grid[ny][nx] = true;
        }
    }
}
