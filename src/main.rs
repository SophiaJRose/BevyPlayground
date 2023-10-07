use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    sprite: SpriteBundle,
    velocity: Velocity,
    state: PlayerState
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

#[derive(Component)]
struct Velocity(Vec2);

enum StateEnum {GROUNDED, JUMPING}

#[derive(Component)]
struct PlayerState(StateEnum);

const GRAVITY: f32 = 1.0;
const MOVE_SPEED: f32 = 6.0;
const JUMP_SPEED: f32 = 20.0;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
           .add_systems(Update, (movement, collision_check));
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
        },
        velocity: Velocity(Vec2::ZERO),
        state: PlayerState(StateEnum::JUMPING)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle { 
            sprite: Sprite { custom_size: Some(Vec2::new(960.0, 48.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, -240.0, 0.0), 
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        }
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(-270.0, -60.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        }
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(270.0, -60.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        }
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, 120.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        }
    });
    commands.spawn((Camera2dBundle::default(), Camera));
}

fn collision_check(ground: Query<(&Sprite, &Transform), With<Ground>>, mut player: Query<(&Sprite, &mut Transform, &mut Velocity, &mut PlayerState), (With<Player>, Without<Ground>)>) {
    for (player_sprite, mut player_transform, mut player_velocity, mut state) in player.iter_mut() {
        let mut any_collisions = false;
        for (ground_sprite, ground_transform) in ground.iter() {
            let player_pos = player_transform.translation;
            let player_size = player_sprite.custom_size.unwrap_or_default();
            let ground_pos = ground_transform.translation;
            let ground_size = ground_sprite.custom_size.unwrap_or_default();
            let collision = collide(player_pos, player_size, ground_pos, ground_size);
            if collision.is_some_and(|c| c == Collision::Top) {
                player_transform.translation = Vec3::new(player_pos.x, ground_pos.y + ((ground_size.y + player_size.y) / 2.0), player_pos.z);
                player_velocity.0 = Vec2::new(player_velocity.0.x, 0.0);
                state.0 = StateEnum::GROUNDED;
                any_collisions = true;
            } else if collision.is_some_and(|c| c == Collision::Bottom) {
                player_transform.translation = Vec3::new(player_pos.x, ground_pos.y - ((ground_size.y + player_size.y) / 2.0), player_pos.z);
                player_velocity.0 = Vec2::new(player_velocity.0.x, f32::min(0.0, player_velocity.0.y));
            } else if collision.is_some_and(|c| c == Collision::Left) {
                player_transform.translation = Vec3::new(ground_pos.x - ((ground_size.x + player_size.x) / 2.0), player_pos.y, player_pos.z);
                player_velocity.0 = Vec2::new(0.0, player_velocity.0.y);
            } else if collision.is_some_and(|c| c == Collision::Right) {
                player_transform.translation = Vec3::new(ground_pos.x + ((ground_size.x + player_size.x) / 2.0), player_pos.y, player_pos.z);
                player_velocity.0 = Vec2::new(0.0, player_velocity.0.y);
            }
        }
        if !any_collisions {
            state.0 = StateEnum::JUMPING;
        }
    }
}

fn movement(mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>, keys: Res<Input<KeyCode>>) {
    for (mut transform, mut velocity, mut state) in player.iter_mut() {
        let player_pos = transform.translation;
        let mut new_x_velocity = 0.0;
        let mut new_y_velocity = velocity.0.y - GRAVITY;
        // Move left or right
        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            new_x_velocity -= MOVE_SPEED;
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right)  {
            new_x_velocity += MOVE_SPEED;
        }
        let jump_just_pressed = keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Up);
        let jump_pressed = keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::Up);
        // Jump
        if jump_just_pressed && matches!(state.0, StateEnum::GROUNDED) {
            new_y_velocity = JUMP_SPEED;
            state.0 = StateEnum::JUMPING;
        }
        // If jump key released before apex, half upward velocity to shorten jump
        if !jump_pressed && new_y_velocity >= 0.0 {
            new_y_velocity /= 2.0;
        }
        velocity.0 = Vec2::new(new_x_velocity, new_y_velocity);
        let player_new_pos = Vec3::new(player_pos.x + velocity.0.x, player_pos.y + velocity.0.y, player_pos.z);
        transform.translation = player_new_pos;
    }
}