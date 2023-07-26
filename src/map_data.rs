use bevy::prelude::*;
use coord_2d::{Coord, Size};
use grid_2d::Grid;
use noise::{Fbm, NoiseFn, Perlin};

const _ASCII_GRAYSCALE_92: &'static str =
    " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const ASCII_GRAYSCALE_70: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const ASCII_GRAYSCALE_10: &'static str = " .:-=+*#%@";
const _ASCII_TERRAIN: &'static str = "^++*~~~~~~..";
const DEFAULT_WIDTH: u32 = 80;
const DEFAULT_HEIGHT: u32 = 45;
const DEFAULT_OFFSET: (f64, f64) = (0.0, 0.0);
const DEFAULT_DISTANCE: f64 = 0.03;
const DEFAULT_SEED: u32 = 0;

#[derive(Resource)]
pub struct MapData {
    pub reverse: bool,
    pub grayscale: String,
    pub grid: Grid<f64>,
    pub sample_offset: (f64, f64),
    pub sample_distance: f64,
    pub noise: Fbm<Perlin>,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            reverse: false,
            grayscale: ASCII_GRAYSCALE_70.to_string(),
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
            grid: Grid::new_default(size),
            sample_offset,
            sample_distance,
            noise: Fbm::new(seed),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        for coord in self.grid.coord_iter() {
            let height = self.noise.get([
                coord.x as f64 * self.sample_distance + self.sample_offset.0,
                coord.y as f64 * self.sample_distance + self.sample_offset.1,
            ]);
            *self.grid.get_checked_mut(coord) = height;
        }
    }

    pub fn get_height_at(&self, coord: Coord) -> Option<f64> {
        self.grid.get(coord).cloned()
    }

    pub fn get_grayscale_at(&self, coord: Coord) -> u8 {
        if let Some(height) = self.grid.get(coord) {
            self.get_grayscale(*height)
        } else {
            0
        }
    }

    fn get_grayscale(&self, height: f64) -> u8 {
        let grayscale = if self.grayscale.len() > 0 {
            self.grayscale.as_bytes()
        } else {
            ASCII_GRAYSCALE_10.as_bytes()
        };

        let value = height.clamp(-1.0, 1.0);
        let m_min = -1 as f64;
        let m_max = 1 as f64;
        let t_min = 0 as f64;
        let t_max = (grayscale.len() - 1) as f64;

        let index = ((value - m_min) / (m_max - m_min) * (t_max - t_min) + t_min) as usize;

        if self.reverse {
            grayscale[index]
        } else {
            grayscale[grayscale.len() - 1 - index]
        }
    }
}
