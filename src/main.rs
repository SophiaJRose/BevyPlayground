use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

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
        app.add_systems(Startup, setup)
           .add_systems(Update, collision_check);
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
            sprite: Sprite { custom_size: Some(Vec2::new(48.0, 48.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, 0.0, 0.0), 
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..default() },
            texture: server.load("TempChar.png"), ..default()
        }
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle { 
            sprite: Sprite { custom_size: Some(Vec2::new(48.0, 48.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, -144.0, 0.0), 
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..default() },
            texture: server.load("TempTile.png"), ..default()
        }
    });
    commands.spawn((Camera2dBundle::default(), Camera));
}

fn collision_check(ground: Query<(&Sprite, &Transform), With<Ground>>, mut player: Query<(&Sprite, &mut Transform), (With<Player>, Without<Ground>)>) {
    for (ground_sprite, ground_transform) in ground.iter() {
        for (player_sprite, mut player_transform) in player.iter_mut() {
            let player_pos = player_transform.translation;
            let player_size = player_sprite.custom_size.unwrap_or_default();
            let ground_pos = ground_transform.translation;
            let ground_size = ground_sprite.custom_size.unwrap_or_default();
            let collision = collide(player_pos, player_size, ground_pos, ground_size);
            if collision.is_some_and(|c| c == Collision::Top) {
                player_transform.translation = Vec3::new(player_pos.x, ground_pos.y + ((ground_size.y + player_size.y) / 2.0), player_pos.z);
            }
        }
    }
}