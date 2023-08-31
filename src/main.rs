#![allow(warnings)]

use bevy::{prelude::*, window::PrimaryWindow};
use rand::{self, Rng};

mod entity;

use entity::*;

const SPRITES_SIZE: [f32; 2] = [160.0, 160.0];
const FIXED_WIDTH: f32 = 560.0;
const FIXED_HEIGTH: f32 = 370.0;

fn main() {
    let (r, g, b) = (240.0, 240.0, 240.0);
    App::new()
        .insert_resource(ClearColor(Color::rgb(r, g, b)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280., 900.).into(),
                resizable: false,
                title: "RPS Royale".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup, spawn_cells))
        .add_systems(Update, (update_rock))
        .run();
}

pub fn spawn_entity(
    context: &mut Commands,
    entity: Mob,
    position: (f32, f32),
    assets: AssetServer,
) {
    let (x, y) = position;
    let sprite = SpriteBundle {
        texture: assets.load(entity.image_path()),
        transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.3)),
        ..Default::default()
    };
    match entity {
        Mob::Rock => context.spawn((sprite, entity, Rock)),
        Mob::Paper => context.spawn((sprite, entity, Paper)),
        Mob::Scissor => context.spawn((sprite, entity, Scissors)),
    };
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_cells(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();
    let mut randomizer = rand::thread_rng();

    let mut fixed_regions = vec![
        (-FIXED_WIDTH..-40.0, 40.0..FIXED_HEIGTH),   // First region
        (40.0..FIXED_WIDTH, 40.0..FIXED_HEIGTH),     // Second region
        (-FIXED_WIDTH..-40.0, -FIXED_HEIGTH..-40.0), // third region
        (40.0..FIXED_WIDTH, -FIXED_HEIGTH..-40.0),   // fourth region
    ];

    [Mob::Rock, Mob::Paper, Mob::Scissor]
        .iter()
        .for_each(move |mob| {
            let region_index = randomizer.gen_range(0..fixed_regions.len());
            let (width_range, height_range) = fixed_regions.remove(region_index);
            let mut randomizer = randomizer.clone();
            (0..20).for_each(|_| {
                let (width, height) = (
                    randomizer.gen_range(width_range.clone()),
                    randomizer.gen_range(height_range.clone()),
                );
                spawn_entity(&mut commands, *mob, (width, height), assets.clone())
            })
        })
}

fn update_rock(mut commands: Commands, query: Query<(&Mob, &Transform)>) {
    let mut closest_allie = None::<f32>;
    let mut closest_enemy = None::<f32>;
    for [(mob, trans), (next, next_trans)] in query.iter_combinations() {
        match (mob, next) {
            // para los enemigos
            (&Mob::Rock, &Mob::Scissor) => {
                let distance = trans.translation.distance(next_trans.translation);
                if let Some(dis) = closest_enemy {
                    if dis < distance {
                        closest_enemy = Some(distance)
                    }
                }
            }
            (&Mob::Scissor, &Mob::Paper) => {}
            (&Mob::Paper, &Mob::Rock) => {}

            // para los aliados
            (&Mob::Rock, &Mob::Rock) => {}
            (&Mob::Scissor, &Mob::Scissor) => {}
            (&Mob::Paper, &Mob::Paper) => {}

            _ => {}
        }
    }
}
