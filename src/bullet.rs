use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::collision::*;
use crate::audio::*;
use crate::player::PlayerInfo;
use crate::app_state::AppState;

const SPEED: f32 = 10.0;


#[derive(Component)]
pub struct Bullet{
    pub velocity: Vec3,
}

fn move_bullet(mut bullets: Query<(&mut Bullet, &mut Velocity)>){
    for mut bullet in bullets.iter_mut(){
        bullet.1.linvel = -bullet.0.velocity * SPEED;
    }
}

//fn spawn_bullet()

fn bullet_collision(
    mut bullets: Query<(Entity, &CollisionTag), With<Bullet>>,
    player_info: Res<PlayerInfo>,
    mut commands: Commands,
    audio_controller: Res<AudioController>,
    audio: Res<Audio>,
){

    for (entity, tag) in bullets.iter_mut(){
        println!("{:?}", tag.other.kind);
        match tag.other.kind {
            CollidableKind::Enemy | CollidableKind::Wall => {
                if let Some(slam) = audio_controller.get_handle("slam"){
                    audio.play_spatial_with_settings(
                        slam.handle, PlaybackSettings::ONCE.with_volume(0.5),
                        Transform::from_translation(player_info.position),
                        4.0,
                        tag.this.transform.translation);
                }
                commands.entity(entity).despawn();
            }
                _ => {}
        }
    }
}


pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems((
            move_bullet.in_set(OnUpdate(AppState::InGame)),
            bullet_collision.in_set(OnUpdate(AppState::InGame)),
        ));
    }
}