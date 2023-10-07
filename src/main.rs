use bevy::prelude::*;

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    sprite: SpriteBundle
}

#[derive(Bundle)]
struct GroundBundle {
    marker: Ground,
    sprite: SpriteBundle
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Camera;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        marker: Player,
        sprite: SpriteBundle { 
            transform: Transform { translation: Vec3::new(0.0, 0.0, 0.0), 
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..default() },
            texture: server.load("TempChar.png"), ..default()
        }
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle { 
            transform: Transform { translation: Vec3::new(0.0, -144.0, 0.0), 
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..default() },
            texture: server.load("TempTile.png"), ..default()
        }
    });
    commands.spawn((Camera2dBundle::default(), Camera));
}