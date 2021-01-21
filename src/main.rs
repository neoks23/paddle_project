use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(Scoreboard { score: 0 })
        .add_resource(ClearColor(Color::rgb(0.0, 0.5, 0.0)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .add_system(paddle_collision_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

struct Paddle {
    speed: f32,
    go_back_state: bool,
}
struct Scoreboard {
    score: usize,
}

enum Collider {
    Solid,
    Scorable,
    Paddle,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        // paddle
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .with(Paddle {
            speed: 500.0,
            go_back_state: false,
        })
        .with(Collider::Paddle)
        .spawn(TextBundle {
            text: Text {
                font: asset_server.load("../../bevyProject/bevy/assets/fonts/FiraSans-Bold.ttf"),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    font_size: 40.0,
                    ..Default::default()
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // left
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.y + wall_thickness, 30.0)),
            ..Default::default()
        })
        .with(Collider::Solid);
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction.x * paddle.speed;
        translation.y += time.delta_seconds() * direction.y * paddle.speed;
        // bound the paddle within the walls
        if (paddle.go_back_state) {
            translation.y += 300.0;
        }
    }
}
fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}
fn paddle_collision_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut scoreboard: ResMut<Scoreboard>,
    mut paddle_query: Query<(&mut Paddle, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut paddle, paddle_transform, sprite) in paddle_query.iter_mut() {
        let paddle_size = sprite.size;
        let velocity = &mut paddle.speed;

        if (paddle.go_back_state) {
            paddle.go_back_state = false;
        }
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                paddle_transform.translation,
                paddle_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Solid = *collider {
                    scoreboard.score += 1;
                    paddle.go_back_state = true;
                    break;
                }
            }
        }
    }
}
