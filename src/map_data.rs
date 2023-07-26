use bevy::prelude::*;
use coord_2d::Size;
use grid_2d::Grid;
use noise::{Fbm, Perlin, NoiseFn};

const ASCII_GRAYSCALE: &'static str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const DEFAULT_WIDTH: u32 = 50;
const DEFAULT_HEIGHT: u32 = 50;
const DEFAULT_OFFSET: (f64, f64) = (0.6, 0.6);
const DEFAULT_DISTANCE: f64 = 0.03;
const DEFAULT_SEED: u32 = 0;


#[derive(Resource)]
pub struct MapData {
    pub grayscale: String,
    pub grid: Grid<u8>,
    pub sample_offset: (f64, f64),
    pub sample_distance: f64,
    pub noise: Fbm<Perlin>,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            grayscale: ASCII_GRAYSCALE.to_string(),
            grid: Grid::new_default(Size::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)),
            sample_offset: DEFAULT_OFFSET,
            sample_distance: DEFAULT_DISTANCE,
            noise: Fbm::new(DEFAULT_SEED),
        }
    }
}

impl MapData {
    pub fn new(size: Size, seed: u32, sample_offset: (f64, f64), sample_distance: f64) -> Self {
        Self {
            grayscale: ASCII_GRAYSCALE.to_string(),
            grid: Grid::new_default(size),
            sample_offset,
            sample_distance,
            noise: Fbm::new(seed),
        }
    }

    pub fn update(&mut self) {
        for coord in self.grid.coord_iter() {
            let height = self
                .noise
                .get([
                    coord.x as f64 * self.sample_distance + self.sample_offset.0,
                    coord.y as f64 * self.sample_distance + self.sample_offset.1,
                ]);
            *self.grid.get_checked_mut(coord) = self.get_grayscale(height);
        }
    }

    fn get_grayscale(&self, value: f64) -> u8 {
        let value = value.clamp(-1.0, 1.0);
        let m_min = -1 as f64;
        let m_max = 1 as f64;
        let t_min = 0 as f64;
        let t_max = (self.grayscale.len() - 1) as f64;

        let index = ((value - m_min) / (m_max - m_min) * (t_max - t_min) + t_min) as usize;
        // println!("[{}, {}] {}, -> {}", x, y, value, index);
        self.grayscale.as_bytes()[index]
    }
}