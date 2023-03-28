use audio::{AddHandle, GetHandle};
use bevy::{prelude::*, window::PrimaryWindow};
pub mod collision;
pub mod audio;
pub mod bullet;
pub mod enemy;
pub mod player_info;
use bevy_rapier3d::{prelude::*};
use std::env;

/* #region Test */

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Player {
    speed: f32,
}



pub struct SetupPlugin;
/* #endregion */

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    // this method needs to be inside main() method
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(SetupPlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .run();
}

//setup scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0, subdivisions: 0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        })
        .insert(Collider::cuboid(25.0, 0.1, 25.0));

    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            transform: Transform::from_xyz(0.0, 1.0, 1.0),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        },
        collision::Collidable{kind: collision::CollidableKind::Enemy},
        enemy::Enemy{}
        
    ));
    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 10.0, 50.0))),
            transform: Transform::from_xyz(25.0, 0.0, 0.0),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        })
        .insert(collision::Collidable{kind: collision::CollidableKind::Wall})
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction{
            coefficient:0.0,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(Collider::cuboid(0.5, 10.0, 25.0));

    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 10.0, 50.0))),
            transform: Transform::from_xyz(-25.0, 0.0, 0.0),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        })
        .insert(collision::Collidable{kind: collision::CollidableKind::Wall})
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction{
            coefficient:0.0,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(Collider::cuboid(0.5, 10.0, 25.0));

    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(51.0, 10.0, 1.0))),
            transform: Transform::from_xyz(0.0, 0.0, 25.0),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        })
        .insert(collision::Collidable{kind: collision::CollidableKind::Wall})
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction{
            coefficient:0.0,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(Collider::cuboid(25.0, 10.0, 0.5));

    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(51.0, 10.0, 1.0))),
            transform: Transform::from_xyz(0.0, 0.0, -25.0),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        })
        .insert(collision::Collidable{kind: collision::CollidableKind::Wall})
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction{
            coefficient:0.0,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(Collider::cuboid(25.0, 10.0, 0.5));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn load_assets(commands: Commands, server: Res<AssetServer>, mut audio_controller: ResMut<audio::AudioController>){
    let player_mesh = server.load("robot.glb#Scene0");
    audio_controller.add_handle("bonk", server.load("sounds/bonk-gavin6049.ogg"));
    audio_controller.add_handle("explosion", server.load("sounds/explosion-prof-mudkip.ogg"));
    audio_controller.add_handle("gunshot", server.load("sounds/gunshot-jofae.ogg"));
    audio_controller.add_handle("inferno", server.load("sounds/inferno-hvrl.ogg"));
    audio_controller.add_handle("laser", server.load("sounds/laser-daleonfire.ogg"));
    audio_controller.add_handle("music", server.load("sounds/music.ogg"));
    audio_controller.add_handle("slam", server.load("sounds/slam-jofae.ogg"));

    create_player(player_mesh, commands);
}

fn create_player(player_mesh: Handle<Scene>, mut commands: Commands) {
    let mut trans = Transform::from_translation(Vec3::new(0.0, 1.0,0.0)).looking_at(Vec3::NEG_Z, Vec3::Y);
    trans = trans.with_scale(Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    });
    commands.spawn((
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::default(),
        SpatialBundle::default(),
        Collider::cuboid(0.5, 0.5, 0.5),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        GravityScale(0.0),
        collision::Collidable{kind: collision::CollidableKind::Player},
        Player{speed: 5.0}
    )).with_children(|children| {
        children.spawn(SceneBundle {
            scene: player_mesh,
            transform: trans,
            ..default()
        });
        children.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
            ..default()
        });
    });
}

//YAY QUERY GO BRR
fn rotate_player(
    player_children: Query<&Children, With<Player>>,
    mut transforms: Query<&mut Transform, Without<Camera>>,
    cameras: Query<(&mut Camera, &mut GlobalTransform)>,
    window: Query<(&Window, With<PrimaryWindow>)>,
    player_info: Res<player_info::PlayerInfo>,
    context: Res<RapierContext>,
) {

    let Ok(primary_window) = window.get_single() else{return;};

    let mouse_pos = primary_window.0.cursor_position().unwrap_or_default();

    let mut hit_point = Vec3::ZERO;

    for camera in cameras.iter() {
        let ray = camera.0.viewport_to_world(camera.1, mouse_pos);

        if let Some((_, intersection)) = context.cast_ray_and_get_normal(
            ray.unwrap().origin,
            ray.unwrap().direction,
            Real::MAX,
            false,
            QueryFilter::default(),
        ) {
            hit_point = intersection.point;
        }
    }

    for child in player_children.iter() {
        for entity in child.iter() {
            if let Ok(mut player_mesh) = transforms.get_mut(*entity) {
                let vec_between = hit_point - player_info.position;
                let angle = vec_between.x.atan2(vec_between.z);
                player_mesh.rotation = Quat::from_axis_angle(Vec3::Y, angle);
            }
        }
    }
}

// Player Movement Input
fn move_player(
    mut player: Query<(&mut Velocity, &mut Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    for (mut vel, player) in player.iter_mut(){
        let movement_vec = Vec3::new(
            keyboard_input.pressed(KeyCode::A) as i32 as f32 * 1.0
                + keyboard_input.pressed(KeyCode::D) as i32 as f32 * -1.0,
            0.0,
            keyboard_input.pressed(KeyCode::W) as i32 as f32 * 1.0
                + keyboard_input.pressed(KeyCode::S) as i32 as f32 * -1.0,
        );
        if movement_vec != Vec3::ZERO{
            vel.linvel = movement_vec * timer.delta_seconds() * player.speed * Vec3::new(100.0,0.0,100.0);
        }else{
            vel.linvel = Vec3::ZERO;
        }
    }
}


// Player Shooting Input
fn shoot_bullet(
    player_info: Res<player_info::PlayerInfo>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        
        println!("{}", player_info.forward);
        let mut shooting_point = player_info.position.clone();
        shooting_point.y += 1.0;
        shooting_point += 2.0*-player_info.forward;
        commands
            .spawn((
                RigidBody::Dynamic,
                bullet::Bullet{
                    velocity: player_info.forward,
                },
                Collider::cuboid(0.05, 0.05, 0.25),
                collision::Collidable{kind: collision::CollidableKind::Bullet},
                Velocity::default(),
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.5))),
                    material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                    transform: Transform::from_translation(shooting_point).looking_at(player_info.forward+player_info.position, Vec3::Y),
                    ..default()
                },

        ));
    }
}

fn update_player_info(
    player: Query<& Transform, With<Player>>,
    player_children: Query<&Children, With<Player>>,
    transforms: Query<&mut Transform, (Without<Player>, Without<Camera>)>,
    mut player_info: ResMut<player_info::PlayerInfo>,
) {
    for p in player.iter(){
        player_info.position = p.translation;
    }

    for child in player_children.iter() {
        for entity in child.iter() {
            if let Ok(player_mesh) = transforms.get(*entity) {
                player_info.rotation = player_mesh.rotation;
                player_info.forward = player_mesh.forward();
            }
        }
    }
}

fn player_collision(
    mut player: Query<Entity, (With<Player>, With<collision::CollisionTag>)>,
    audio_controller: Res<audio::AudioController>,
    audio: Res<Audio>,
    collision_info: Res<collision::CollisionInfo>
){
    for entity in player.iter_mut(){
        if let Some(collision_list) = collision_info.get(&entity){
            for collision in collision_list {
                match collision.other.kind {
                    collision::CollidableKind::Wall => {
                        if let Some(bonk) =  audio_controller.get_handle("bonk"){
                           audio.play(bonk.handle);
                       }
                    }
                    collision::CollidableKind::Enemy =>{
                    }
                    _ => {}
                }
            }
        } 
        else{ continue; };
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(player_info::PlayerInfo {
                position: Vec3::ZERO,
                forward: Vec3::NEG_Z,
                rotation: Quat::IDENTITY,
            })
            .add_startup_system(load_assets)
            .add_startup_system(setup)
            .add_system(update_player_info)
            .add_system(rotate_player)
            .add_system(move_player)
            .add_system(player_collision)
            .add_system(shoot_bullet);
    }
}
