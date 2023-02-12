use std::ops::Mul;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
/*




NOTE TO FUTURE SELF:
Fix Query

INVESTIGATE RAPIER





*/

/* #region Test */

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Player {
    speed: f32,
}
pub struct SetupPlugin;
/* #endregion */

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(SetupPlugin)
        .run();
}

//setup scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..default()
    });
    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            material: materials.add(Color::rgb(0.0, 1.0, 1.0).into()),
            ..default()
        })
        .insert(Collider::cuboid(1.0, 1.0, 1.0));
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

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut cameras: Query<(&Parent, &mut Transform), With<Camera>>,
    mut players: Query<&GlobalTransform>,
) {
    /*
    for mut player in players.iter_mut(){
        if keyboard_input.pressed(KeyCode::Space){
            player.camera.transform.translation.y += 0.1;
        }
        if keyboard_input.pressed(KeyCode::LShift){
            player.camera.transform.translation.y -= 0.1;
            println!("{}", player.camera.transform.translation.y);
        }
    } */
    // we can use parent to check if player i guess??
    for (parent, mut camera_transform) in cameras.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            camera_transform.translation.y += 0.1;
            camera_transform.rotate_local_z(0.1);
            println!("{}", camera_transform.translation);
        }
        if keyboard_input.pressed(KeyCode::LShift) {
            camera_transform.translation.y -= 0.1;
        }
    }
}

fn load_models(mut commands: Commands, server: Res<AssetServer>) {
    let player_mesh = server.load("robot.glb#Scene0");
    create_player(player_mesh, commands);
}

fn create_player(player_mesh: Handle<Scene>, mut commands: Commands) {
    let mut trans = Transform::from_translation(Vec3::ZERO);
    trans = trans.with_scale(Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    });
    let _player = commands
        .spawn(RigidBody::KinematicPositionBased)
        .with_children(|children| {
            children.spawn(SceneBundle {
                scene: player_mesh,
                transform: trans,
                ..default()
            });
            children.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
                ..default()
            });
        })
        .insert(SpatialBundle::default())
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(KinematicCharacterController::default())
        .insert(Player { speed: 5.0 });
}

//YAY QUERY GO BRR
fn rotate_player(
    players: Query<&Children, With<Player>>,
    windows: Res<Windows>,
    mut transforms: Query<&mut Transform, Without<Camera>>,
) {
    let mouse_pos = windows
        .get_primary()
        .unwrap()
        .cursor_position()
        .unwrap_or_default();
    // for raycasting install rapier.
    //println!("{}", mouse_pos);
    for children in players.iter() {
        for entity in children.iter() {
            if let Ok(mut player_body) = transforms.get_mut(*entity) {
                player_body.rotate_local_y(0.1);
            }
        }
    }
}

// Player Input
fn move_player(
    mut player_controllers: Query<(&mut KinematicCharacterController, &mut Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    timer: Res<Time>,
) {
    for player_controller in player_controllers.iter_mut() {
        let mut controller = player_controller.0;
        let player_struct = player_controller.1;
        let mut movement_vec = Vec3::new(
            keyboard_input.pressed(KeyCode::A) as i32 as f32 * 1.0
                + keyboard_input.pressed(KeyCode::D) as i32 as f32 * -1.0,
            0.0,
            keyboard_input.pressed(KeyCode::W) as i32 as f32 * 1.0
                + keyboard_input.pressed(KeyCode::S) as i32 as f32 * -1.0,
        );
        movement_vec *= player_struct.speed;
        movement_vec *= timer.delta_seconds();
        controller.translation = Some(movement_vec);
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(load_models)
            .add_startup_system(setup)
            .add_system(camera_movement)
            .add_system(rotate_player)
            .add_system(move_player);
    }
}
