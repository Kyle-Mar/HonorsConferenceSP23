use std::f32::consts::PI;
use std::time::Duration;
use rand::prelude::*;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_rapier3d::prelude::{Velocity, RigidBody, CoefficientCombineRule, ActiveEvents, Friction, Collider};
use crate::collision::{*, self};
use crate::player::PlayerInfo;
use crate::app_state::AppState;


#[derive(Component)]
pub struct Enemy{
}

#[derive(Resource, Clone)]
pub struct EnemyMeshScene(pub Handle<Scene>);


fn move_enemy(
    mut enemies: Query<(&mut Enemy, &mut Transform, &mut Velocity)>,
    player_info: Res<PlayerInfo>,
){
    for (_, mut transform, mut velocity) in enemies.iter_mut() {
        let vec_between = transform.translation - player_info.position;
        let angle = vec_between.x.atan2(vec_between.z);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, angle+PI);
        let forward = transform.forward();
        velocity.linvel = -forward;
    }
}

fn spawn_enemies(
    mut commands: Commands,
    enemy_mesh: ResMut<EnemyMeshScene>,
){
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-23..23);
    let z = rng.gen_range(-23..23);
    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(SpatialBundle{..default()})
        .insert(SceneBundle{
            scene: enemy_mesh.0.clone(),
            transform: Transform::from_xyz(x as f32, 0.5, z as f32),
            ..default()
        })
        /*.insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            transform: Transform::from_xyz(x as f32, 0.5, z as f32),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            ..default()
        })*/
        .insert(collision::Collidable{kind: collision::CollidableKind::Enemy})
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Friction{
            coefficient:0.0,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(Enemy{})
        .insert(Velocity::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
}

fn enemy_collision(
    mut enemies: Query<(Entity, &CollisionTag), With<Enemy>>,
    mut commands: Commands
    //player_info: Res<PlayerInfo>,
){

    for (entity, tag) in enemies.iter_mut(){
        println!("{:?}", tag.other.kind);
        match tag.other.kind {
            CollidableKind::Bullet => {
                println!("{:?}", entity);
                commands.entity(entity).despawn_recursive();
                //commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}




pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_enemies.run_if(in_state(AppState::InGame)).run_if(on_timer(Duration::from_secs(1))),
            move_enemy.in_set(OnUpdate(AppState::InGame)),
            enemy_collision.in_set(OnUpdate(AppState::InGame)),
        ));
    }
}