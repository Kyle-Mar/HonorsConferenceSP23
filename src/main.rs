use app_state::AppState;
use audio::{AddHandle, GetHandle};
use bevy::{prelude::*, window::PrimaryWindow, utils::Duration};
pub mod collision;
pub mod audio;
pub mod bullet;
pub mod enemy;
pub mod player;
pub mod app_state;
pub mod menu;
pub mod hud;

use bevy_rapier3d::{prelude::*};
use player::{DamageCooldown, PlayerInfo, PlayerMeshScene};
use std::{env,};

/* #region Test */

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);




pub struct SetupPlugin;
/* #endregion */

fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    // this method needs to be inside main() method
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin{
            always_on_top: true,
            enabled: true,
            ..default()

        })
        .add_plugin(SetupPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(hud::HudPlugin)
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

fn load_assets(mut commands: Commands, server: Res<AssetServer>, mut audio_controller: ResMut<audio::AudioController>){
    let player_mesh = server.load("robot.glb#Scene0");
    let enemy_mesh = server.load("eyeball.glb#Scene0");
    audio_controller.add_handle("bonk", server.load("sounds/bonk-gavin6049.ogg"));
    audio_controller.add_handle("explosion", server.load("sounds/explosion-prof-mudkip.ogg"));
    audio_controller.add_handle("gunshot", server.load("sounds/gunshot-jofae.ogg"));
    audio_controller.add_handle("inferno", server.load("sounds/inferno-hvrl.ogg"));
    audio_controller.add_handle("laser", server.load("sounds/laser-daleonfire.ogg"));
    audio_controller.add_handle("music", server.load("sounds/music.ogg"));
    audio_controller.add_handle("slam", server.load("sounds/slam-jofae.ogg"));
    commands.insert_resource(player::PlayerMeshScene(player_mesh));
    commands.insert_resource(enemy::EnemyMeshScene(enemy_mesh));
    //create_player(player_mesh, commands);
}

fn create_player(player_mesh: Res<PlayerMeshScene>, mut commands: Commands) {
    let mut timer = Timer::new(Duration::from_secs(1), TimerMode::Repeating);
    commands.spawn((
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::default(),
        SpatialBundle{global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed: ComputedVisibility::default(),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0))},
        Collider::cuboid(0.5, 0.5, 0.5),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        GravityScale(0.0),
        player::DamageCooldown{timer: timer},
        collision::Collidable{kind: collision::CollidableKind::Player},
        player::Player{speed: 5.0, health: 10}
    )).with_children(|children| {
        children.spawn(SceneBundle {
            scene: player_mesh.0.clone(),
            transform: Transform::default()
                                .looking_at(Vec3::NEG_Z, Vec3::Y)
                                .with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        });
        children.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
            ..default()
        });
    });
}

//YAY QUERY GO BRR

fn print_test(){
    println!("HI");
}

fn setup_state(
    mut next_state: ResMut<NextState<app_state::AppState>>,
    cur_state: Res<State<app_state::AppState>>,
    keys: Res<Input<KeyCode>>,
){
    if(keys.just_pressed(KeyCode::Space) && cur_state.0 == app_state::AppState::MainMenu){
        next_state.set(app_state::AppState::InGame);
    }
}


fn cleanup_scene(
    mut commands: Commands,
    mut entities: Query<Entity, Without<Window>>,
){
    for mut entity in entities.iter_mut(){
        commands.entity(entity).despawn();
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<app_state::AppState>();
        app
            .insert_resource(PlayerInfo {
                position: Vec3::ZERO,
                forward: Vec3::NEG_Z,
                rotation: Quat::IDENTITY,
            })
            .add_system(setup_state)
            .add_system(load_assets.on_startup())
            .add_system(cleanup_scene.in_schedule(OnExit(app_state::AppState::InGame)))
            .add_system(cleanup_scene.in_schedule(OnExit(app_state::AppState::MainMenu)))
            .add_system(create_player.in_schedule(OnEnter(app_state::AppState::InGame)))
            .add_system(setup.in_schedule(OnEnter(app_state::AppState::InGame)))
            .add_system(print_test.in_schedule(OnEnter(app_state::AppState::MainMenu)));
    }
}
