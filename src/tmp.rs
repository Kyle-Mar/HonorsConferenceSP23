use bevy::{prelude::*};

/* #region Test */

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Player;
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>
){
    
    let player_mesh = server.load("robot.glb#Scene0");
    let mut trans = Transform::from_translation(Vec3::ZERO);
    trans = trans.with_scale(Vec3{x: 0.1, y: 0.1, z: 0.1});
    commands.spawn(SceneBundle{
        scene: player_mesh,
        transform: trans,
        ..default()
    }).insert(Player);

    commands.spawn(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(1.0,0.0,0.0).into()),
        ..default()
    });

    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(PlayerCamera);

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

impl Plugin for SetupPlugin{
    fn build(&self, app: &mut App){
        app
        .add_startup_system(setup);
    }
}