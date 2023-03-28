use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::collision::*;
use crate::audio::*;
use crate::player_info::*;

const SPEED: f32 = 10.0;


#[derive(Component)]
pub struct Enemy{
}

fn move_enemy(
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    player_info: Res<PlayerInfo>,
    timer: Res<Time>,
){
    for (_, mut transform) in enemies.iter_mut() {
        let vec_between = transform.translation - player_info.position;
        let angle = vec_between.x.atan2(vec_between.z);
        transform.rotation = Quat::from_axis_angle(Vec3::Y, angle);
        let forward = transform.forward();
        transform.translation += forward * timer.delta_seconds();
    }
}
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(move_enemy);
    }
}