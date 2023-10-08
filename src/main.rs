use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    sprite: SpriteBundle,
    velocity: Velocity,
    state: PlayerState,
    run_timer: RunTimer,
    wall_jump_timer: WallJumpTimer
}

#[derive(Bundle)]
struct GroundBundle {
    marker: Ground,
    sprite: SpriteBundle,
    death_plane: DeathPlane
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct DeathPlane(bool);

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct RunTimer(f32);

#[derive(Component)]
struct WallJumpTimer(f32);

#[derive(Copy, Clone)]
enum StateEnum {Grounded, Jumping, WallSliding, WallJumping}

#[derive(Component)]
struct PlayerState(StateEnum);

#[derive(Bundle)]
struct SavePointBundle {
    marker: SavePoint,
    transform: Transform,
    velocity: Velocity,
    state: PlayerState,
    run_timer: RunTimer,
    wall_jump_timer: WallJumpTimer
}

#[derive(Component)]
struct SavePoint;

#[derive(Default)]
struct ButtonState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool
}

#[derive(Resource, Default)]
struct InputState {
    left: ButtonState,
    right: ButtonState,
    jump: ButtonState,
    save: ButtonState,
    load: ButtonState
}

const GRAVITY: f32 = 1.0;
const MOVE_SPEED: f32 = 6.0;
const RUN_SPEED: f32 = 9.0;
const JUMP_SPEED: f32 = 20.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
           .add_systems(Startup, setup)
           .add_systems(Update, (input_handling, movement, collision_check, savepoint));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
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
        state: PlayerState(StateEnum::Jumping),
        run_timer: RunTimer(0.0),
        wall_jump_timer: WallJumpTimer(0.0)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle { 
            sprite: Sprite { custom_size: Some(Vec2::new(960.0, 48.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, -240.0, 0.0), 
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(-270.0, -60.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(270.0, -60.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(240.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, 120.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(24.0, 240.0)), ..default() },
            transform: Transform { translation: Vec3::new(510.0, 120.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempWall.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(24.0, 240.0)), ..default() },
            transform: Transform { translation: Vec3::new(-510.0, 120.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempWall.png"), ..default()
        },
        death_plane: DeathPlane(false)
    });
    commands.spawn(GroundBundle {
        marker: Ground,
        sprite: SpriteBundle {
            sprite : Sprite { custom_size: Some(Vec2::new(2400.0, 24.0)), ..default() },
            transform: Transform { translation: Vec3::new(0.0, -1200.0, 0.0),
                                   scale: Vec3::splat(1.0),
                                   ..default() },
            texture: server.load("TempPlatform.png"),
            visibility: Visibility::Hidden, ..default()
        },
        death_plane: DeathPlane(true)
    });
    commands.spawn(SavePointBundle {
        marker: SavePoint,
        transform: Transform::default(),
        velocity: Velocity(Vec2::default()),
        state: PlayerState(StateEnum::Jumping),
        run_timer: RunTimer(0.0),
        wall_jump_timer: WallJumpTimer(0.0)
    });
    commands.spawn((Camera2dBundle::default(), Camera));
}

fn collision_check(ground: Query<(&Sprite, &Transform, &DeathPlane), With<Ground>>, mut player: Query<(&Sprite, &mut Transform, &mut Velocity, &mut PlayerState), (With<Player>, Without<Ground>)>) {
    for (player_sprite, mut player_transform, mut player_velocity, mut state) in player.iter_mut() {
        let mut any_landing_collisions = false;
        let mut any_wall_collisions = false;
        for (ground_sprite, ground_transform, death_plane) in ground.iter() {
            let player_pos = player_transform.translation;
            let player_size = player_sprite.custom_size.unwrap_or_default();
            let ground_pos = ground_transform.translation;
            let ground_size = ground_sprite.custom_size.unwrap_or_default();
            let collision = collide(player_pos, player_size, ground_pos, ground_size);
            if collision.is_some() && death_plane.0 { 
                player_transform.translation = Vec3::ZERO;
                player_velocity.0 = Vec2::ZERO;
                state.0 = StateEnum::Jumping;
            } else if collision.is_some_and(|c| c == Collision::Top) {
                player_transform.translation = Vec3::new(player_pos.x, ground_pos.y + ((ground_size.y + player_size.y) / 2.0), player_pos.z);
                player_velocity.0 = Vec2::new(player_velocity.0.x, f32::max(0.0, player_velocity.0.y));
                state.0 = StateEnum::Grounded;
                any_landing_collisions = true;
            } else if collision.is_some_and(|c| c == Collision::Bottom) {
                player_transform.translation = Vec3::new(player_pos.x, ground_pos.y - ((ground_size.y + player_size.y) / 2.0), player_pos.z);
                player_velocity.0 = Vec2::new(player_velocity.0.x, f32::min(0.0, player_velocity.0.y));
            } else if collision.is_some_and(|c| c == Collision::Left) {
                player_transform.translation = Vec3::new(ground_pos.x - ((ground_size.x + player_size.x) / 2.0), player_pos.y, player_pos.z);
                player_velocity.0 = Vec2::new(0.1, player_velocity.0.y * 0.8);
                state.0 = StateEnum::WallSliding;
                any_wall_collisions = true;
            } else if collision.is_some_and(|c| c == Collision::Right) {
                player_transform.translation = Vec3::new(ground_pos.x + ((ground_size.x + player_size.x) / 2.0), player_pos.y, player_pos.z);
                player_velocity.0 = Vec2::new(-0.1, player_velocity.0.y * 0.8);
                state.0 = StateEnum::WallSliding;
                any_wall_collisions = true;
            }
        }
        if !any_landing_collisions && !any_wall_collisions {
            state.0 = StateEnum::Jumping;
        }
    }
}

fn movement(mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerState, &mut RunTimer, &mut WallJumpTimer), With<Player>>, input: Res<InputState>) {
    for (mut transform, mut velocity, mut state, mut run_timer, mut wall_jump_timer) in player.iter_mut() {
        let player_pos = transform.translation;
        let mut new_x_velocity = velocity.0.x;
        let mut new_y_velocity = velocity.0.y - if matches!(state.0, StateEnum::WallSliding) {GRAVITY / 2.0} else {GRAVITY};
        // Move left or right
        if input.left.pressed {
            if run_timer.0 <= 30.0 && !matches!(state.0, StateEnum::WallJumping) {
                new_x_velocity = -MOVE_SPEED;
            } else {
                new_x_velocity = -RUN_SPEED;
            }
        }
        if input.right.pressed {
            if run_timer.0 <= 30.0 && !matches!(state.0, StateEnum::WallJumping) {
                new_x_velocity = MOVE_SPEED;
            } else {
                new_x_velocity = RUN_SPEED;
            }
        }
        if input.left.pressed == input.right.pressed {
            new_x_velocity = 0.0;
            run_timer.0 = 0.0;
        } else if matches!(state.0, StateEnum::Grounded) {
            run_timer.0 += 1.0;
        }
        // Jump
        if input.jump.just_pressed && matches!(state.0, StateEnum::Grounded) {
            new_y_velocity = JUMP_SPEED;
            state.0 = StateEnum::Jumping;
        }
        // Wall Jump
        if input.jump.just_pressed && matches!(state.0, StateEnum::WallSliding) {
            // Will be either 1 or -1 depending on whether velocity is positive or negative 
            velocity.0.x = RUN_SPEED * velocity.0.x.signum() * -1.0;
            new_y_velocity = JUMP_SPEED;
            state.0 = StateEnum::WallJumping;
            wall_jump_timer.0 = 15.0;
        }
        // If jump key released before apex, half upward velocity to shorten jump
        if !input.jump.pressed && new_y_velocity >= 0.0 {
            new_y_velocity /= 2.0;
        }
        if wall_jump_timer.0 > 0.0 {
            wall_jump_timer.0 -= 1.0;
            velocity.0 = Vec2::new(velocity.0.x, new_y_velocity);
        } else {
            velocity.0 = Vec2::new(new_x_velocity, new_y_velocity);
        }
        let player_new_pos = Vec3::new(player_pos.x + velocity.0.x, player_pos.y + velocity.0.y, player_pos.z);
        transform.translation = player_new_pos;
    }
}

fn savepoint(mut player: Query<(&mut Transform, &mut Velocity, &mut PlayerState, &mut RunTimer, &mut WallJumpTimer), (With<Player>, Without<SavePoint>)>, mut save_point: Query<(&mut Transform, &mut Velocity, &mut PlayerState, &mut RunTimer, &mut WallJumpTimer), (With<SavePoint>, Without<Player>)>, input: Res<InputState>) {
    for (mut player_transform, mut player_velocity, mut player_player_state, mut player_run_timer, mut player_wall_jump_timer) in player.iter_mut() {
        let (mut save_transform, mut save_velocity, mut save_player_state, mut save_run_timer, mut save_wall_jump_timer) = save_point.single_mut();
        if input.save.just_pressed {
            save_transform.translation = player_transform.translation;
            save_velocity.0 = player_velocity.0;
            save_player_state.0 = player_player_state.0;
            save_run_timer.0 = player_run_timer.0;
            save_wall_jump_timer.0 = player_wall_jump_timer.0;
        }
        if input.load.just_pressed {
            player_transform.translation = save_transform.translation;
            player_velocity.0 = save_velocity.0;
            player_player_state.0 = save_player_state.0;
            player_run_timer.0 = save_run_timer.0;
            player_wall_jump_timer.0 = save_wall_jump_timer.0;
        }
    }
}

fn input_handling(mut input: ResMut<InputState>, keys: Res<Input<KeyCode>>) {
    // Left
    input.left.pressed = keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left);
    input.left.just_pressed = keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left);
    input.left.just_released = keys.just_released(KeyCode::A) || keys.just_released(KeyCode::Left);
    // Right
    input.right.pressed = keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right);
    input.right.just_pressed = keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right);
    input.right.just_released = keys.just_released(KeyCode::D) || keys.just_released(KeyCode::Right);
    // Jump
    input.jump.pressed = keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::Space);
    input.jump.just_pressed = keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::Space);
    input.jump.just_released = keys.just_released(KeyCode::W) || keys.just_released(KeyCode::Up) || keys.just_released(KeyCode::Space);
    // Save
    input.save.pressed = keys.pressed(KeyCode::Q);
    input.save.just_pressed = keys.just_pressed(KeyCode::Q);
    input.save.just_released = keys.just_released(KeyCode::Q);
    // Load
    input.load.pressed = keys.pressed(KeyCode::E);
    input.load.just_pressed = keys.just_pressed(KeyCode::E);
    input.load.just_released = keys.just_released(KeyCode::E);
}