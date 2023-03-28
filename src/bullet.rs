use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::collision::*;
use crate::audio::*;
use crate::player_info::*;

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
    mut bullets: Query<Entity, (With<Bullet>, With<CollisionTag>)>,
    player_info: Res<PlayerInfo>,
    collision_info: Res<CollisionInfo>,
    mut commands: Commands,
    audio_controller: Res<AudioController>,
    audio: Res<Audio>,
){
    for entity in bullets.iter_mut(){
        if let Some(collision_list) = collision_info.get(&entity){
            for collision in collision_list {
                match collision.other.kind {
                    CollidableKind::Wall | CollidableKind::Enemy => {
                        if let Some(slam) = audio_controller.get_handle("slam"){
                            audio.play_spatial_with_settings(
                                slam.handle, PlaybackSettings::ONCE.with_volume(0.5),
                                Transform::from_translation(player_info.position),
                                4.0,
                                collision.this.transform.translation);
                            //audio.play_with_settings(slam.handle, PlaybackSettings::ONCE.with_volume(0.01));
                        }
                        commands.entity(entity).despawn();
                        break;
                    }
                    _ => {}
                }
            }
        } 
        else{ continue; };
    }
}


pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(move_bullet)
        .add_system(bullet_collision);
    }
}