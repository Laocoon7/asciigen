use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts, egui};
use coord_2d::{Size, Coord};
use grid_2d::Grid;
use noise::{Fbm, Perlin, Seedable, NoiseFn};

#[derive(Resource)]
struct MapData {
    grid: Grid<u8>,
    sample_offset: (f64, f64),
    sample_distance: f64,
    noise: Fbm<Perlin>,
}

impl MapData {
    pub fn new(size: Size, seed: u32, sample_offset: (f64, f64), sample_distance: f64) -> Self {
        Self {
            grid: Grid::new_default(size),
            sample_offset,
            sample_distance,
            noise: Fbm::new(seed),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1920., 1080.].into(),
                title: "AsciiGen".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(MapData::new(Size::new(100, 100), 0, (0.6, 0.6), 0.3))
        .add_systems(Update, (on_grid_changed, draw_screen).chain())
        .run();
}

fn on_grid_changed(map_data: Option<ResMut<MapData>>) {
    let Some(mut map_data) = map_data else { return; };
    if map_data.is_changed() {
        update_grid(&mut map_data);
    }
}

fn draw_screen(
    mut egui_contexts: EguiContexts,
    map_data: ResMut<MapData>,
) {
    egui::TopBottomPanel::top("noise").show(egui_contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Seed: {}", map_data.noise.seed()));
            ui.label(format!("Octaves: {}", map_data.noise.octaves));
            ui.label(format!("Frequency: {}", map_data.noise.frequency));
            ui.label(format!("Lacunarity: {}", map_data.noise.lacunarity));
            ui.label(format!("Persistence: {}", map_data.noise.persistence));
        });
    });
    egui::SidePanel::right("grid").show(egui_contexts.ctx_mut(), |ui| {
        ui.label(format!("Width: {}", map_data.grid.width()));
        ui.label(format!("Height: {}", map_data.grid.height()));
        ui.label(format!("Sample Distance: {}", map_data.sample_distance));
    });
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        let mut output = String::new();
        for y in 0..map_data.grid.height() {
            let mut row = String::new();
            for x in 0..map_data.grid.width() {
                let value = *map_data.grid.get_checked(Coord::new(x as i32, y  as i32)) as char;
                row.push(value);
            }
            output.push_str(&row);
            output.push_str("\n");
        }

        ui.label(output);
    });
}

fn update_grid(map_data: &mut MapData) {
    for coord in map_data.grid.coord_iter() {
        let height = map_data.noise.get([coord.x as f64 + map_data.sample_offset.0, coord.y as f64 + map_data.sample_offset.1]);
        let value = (height * 255.0 % 256.0) as u8;
        *map_data.grid.get_checked_mut(coord) = value;
    }
}