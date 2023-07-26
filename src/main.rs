use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use coord_2d::{Coord, Size};
use grid_2d::Grid;
use noise::{Fbm, MultiFractal, Perlin, Seedable};

pub mod map_data;
use self::map_data::*;

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
        .insert_resource(MapData::default())
        .add_systems(Startup, setup_font)
        .add_systems(Update, draw_screen)
        .run();
}

fn setup_font(mut egui_contexts: EguiContexts) {
    const FONT_NAME: &'static str = "julia_mono";
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        FONT_NAME.to_string(),
        egui::FontData::from_static(include_bytes!("../assets/JuliaMono-Regular.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, FONT_NAME.to_string());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, FONT_NAME.to_string());

    egui_contexts.ctx_mut().set_fonts(fonts);
}

fn draw_screen(mut egui_contexts: EguiContexts, mut map_data: ResMut<MapData>) {
    egui::TopBottomPanel::top("noise").show(egui_contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            let mut seed = map_data.noise.seed();
            let mut octaves = map_data.noise.octaves;
            let mut frequency = map_data.noise.frequency;
            let mut lacunarity = map_data.noise.lacunarity;
            let mut persistence = map_data.noise.persistence;
            const MAX_OCTAVES: usize = Fbm::<Perlin>::MAX_OCTAVES - 2;
            ui.add(egui::Slider::new(&mut seed, 0..=u32::MAX - MAX_OCTAVES as u32).text("Seed"));
            ui.add(egui::Slider::new(&mut octaves, 1..=MAX_OCTAVES).text("Octaves"));
            ui.add(egui::Slider::new(&mut frequency, 0.0..=2.0f64).text("Frequency"));
            ui.add(egui::Slider::new(&mut lacunarity, 1.0..=3.5f64).text("Lacunarity"));
            ui.add(egui::Slider::new(&mut persistence, 0.0..=1.0f64).text("Persistence"));
            map_data.noise = Fbm::new(seed)
                .set_octaves(octaves)
                .set_frequency(frequency)
                .set_lacunarity(lacunarity)
                .set_persistence(persistence);
        });
    });
    egui::SidePanel::right("grid").show(egui_contexts.ctx_mut(), |ui| {
        let mut width = map_data.grid.width();
        let mut height = map_data.grid.height();

        ui.add(egui::Slider::new(&mut width, 1..=100).text("Width"));
        ui.add(egui::Slider::new(&mut height, 1..=100).text("Height"));
        ui.add(
            egui::Slider::new(&mut map_data.sample_distance, 0.0..=1.0f64).text("Sample Distance"),
        );
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut map_data.sample_offset.0).speed(0.01));
            ui.label("Offset X");
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut map_data.sample_offset.1).speed(0.01));
            ui.label("Offset Y");
        });
        ui.checkbox(&mut map_data.reverse, "Reverse");
        ui.vertical(|ui| {
            ui.label("Grayscale:");
            ui.text_edit_singleline(&mut map_data.grayscale);
        });

        map_data.grid = Grid::new_default(Size::new(width, height));
        map_data.update();
    });
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            for y in 0..map_data.grid.height() {
                ui.horizontal(|ui| {
                    for x in 0..map_data.grid.width() {
                        let mut value = String::new();
                        value.push(
                            map_data.get_grayscale_at(Coord::new(x as i32, y as i32)) as char
                        );
                        ui.monospace(value);
                    }
                });
            }
        });
    });
}
