use bevy::{prelude::*};
/*




NOTE TO FUTURE SELF: 
WILL NEED TO SEPARATE PLAYER MESH FROM PLAYER

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
struct Player{
    speed: f32,
}
pub struct SetupPlugin;

/* #endregion */



fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(SetupPlugin)
    .run();
}

//setup scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..default()
    });
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
    mut players: Query<&GlobalTransform>
){
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
    for (parent, mut camera_transform) in cameras.iter_mut(){
        if keyboard_input.pressed(KeyCode::Space){
            camera_transform.translation.y += 0.1;
            camera_transform.rotate_local_z(0.1);
            println!("{}", camera_transform.translation);
        }
        if keyboard_input.pressed(KeyCode::LShift){
            camera_transform.translation.y -= 0.1;
        }
    }
}

fn load_models(
    mut commands: Commands,
    server: Res<AssetServer>,
){
    let player_mesh = server.load("robot.glb#Scene0");
    let mut trans = Transform::from_translation(Vec3::ZERO);
    trans = trans.with_scale(Vec3{x: 0.1, y: 0.1, z: 0.1});
    
    let player = commands.spawn(SceneBundle{
        scene: player_mesh,
        transform: trans,
        ..default()
    }).insert(Player{speed : 5.0,}
    ).with_children(|player|{
        player.spawn(Camera3dBundle{
            transform: Transform::from_xyz(0.0, 50.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
            ..default()
        });
    });
}

//temp
fn rotate_player(mut players: Query<&mut Transform, With<Player>>){
    for mut player_transform in players.iter_mut(){
        player_transform.rotate_local_x(0.1);
        player_transform.rotate_local_y(0.1);
        player_transform.rotate_local_z(0.1);
    }
}

// Player Input
fn move_player(
    mut players: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    timer: Res<Time>
){
    for player in players.iter_mut(){
        let mut player_transform = player.0;
        let player_class = player.1;
        if keyboard_input.pressed(KeyCode::W) {
            player_transform.translation.z += 1.0 * player_class.speed * timer.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A){
            player_transform.translation.x += 1.0 * player_class.speed * timer.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S){
            player_transform.translation.z -= 1.0 * player_class.speed * timer.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D){
            player_transform.translation.x -= 1.0 * player_class.speed * timer.delta_seconds();
        }
    }
}

impl Plugin for SetupPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(load_models)
        .add_startup_system(setup)
        .add_system(camera_movement)
        .add_system(move_player);
    }
}