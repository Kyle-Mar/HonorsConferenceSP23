use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{prelude::*, rapier::crossbeam::channel::tick};
use bevy::utils::Duration;
use crate::{collision::*, audio::*, bullet::*, app_state::*, hud::HealthBar};

#[derive(Component)]
pub struct DamageCooldown{
    pub timer: Timer,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health : i32,
}

#[derive(Resource, Clone)]
pub struct PlayerMeshScene(pub Handle<Scene>);

fn tick_damage_cooldown(
    mut commands: Commands,
    mut damage_cooldowns: Query<(Entity, &mut DamageCooldown)>,
    time: Res<Time>
){
    for (entity, mut damage_cooldown) in damage_cooldowns.iter_mut(){
        damage_cooldown.timer.tick(time.delta());
        if damage_cooldown.timer.just_finished(){
            damage_cooldown.timer.pause();
        }
    }
}

fn rotate_player(
    player_children: Query<&Children, With<Player>>,
    mut transforms: Query<&mut Transform, Without<Camera>>,
    cameras: Query<(&mut Camera, &mut GlobalTransform)>,
    window: Query<(&Window, With<PrimaryWindow>)>,
    player_info: Res<PlayerInfo>,
    context: Res<RapierContext>,
) {

    let Ok(primary_window) = window.get_single() else{return;};

    let mouse_pos = primary_window.0.cursor_position().unwrap_or_default();

    let mut hit_point = Vec3::ZERO;

    for camera in cameras.iter() {
        let ray = camera.0.viewport_to_world(camera.1, mouse_pos);
        if ray.is_none() {
            continue;
        }
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

fn player_damage(
    mut players: Query<(Entity, &mut DamageCooldown, &mut Player)>,
    mut health_bar: Query<(&Node, &mut Style), With<HealthBar>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut col_start_events: EventReader<CollisionStartEvent>,
    mut col_stay_events: EventReader<CollisionStayEvent>,
){
    let Ok(mut player) = players.get_single_mut() 
    else{
        // this really shouldn't happen lol.
        warn!("MORE THAN ONE PLAYER");
        return;
    };
    
    // if the cool down is active ignore we don't want to check collisions.
    if !player.1.timer.paused(){
        return;
    }

    let Ok(mut hb) = health_bar.get_single_mut()
    else{
        return;
    };
    
    for enter_event in col_start_events.iter(){
        if enter_event.0.entity == player.0 || enter_event.1.entity == player.0 { 
            if enter_event.0.kind != CollidableKind::Enemy && enter_event.1.kind != CollidableKind::Enemy{
                continue;
            }
            //reset timer
            player.1.timer.set_duration(Duration::from_secs(1));
            player.1.timer.unpause();
            //decrement health
            player.2.health-=1;
            hb.1.size = Size::new(Val::Percent(((player.2.health as f32) / 10.0 * 100.) as f32), Val::Percent(100.));

            if player.2.health <= 0{
                next_state.set(AppState::MainMenu);
            }
            return;
        } 
    }
    for stay_event in col_stay_events.iter(){
        if stay_event.0.entity == player.0 || stay_event.1.entity == player.0 {
            if stay_event.0.kind != CollidableKind::Enemy && stay_event.1.kind != CollidableKind::Enemy{
                continue;
            }
            // reset timer
            player.1.timer.set_duration(Duration::from_secs(1));
            player.1.timer.unpause();
            //decrement health
            player.2.health-=1;
            hb.1.size = Size::new(Val::Percent(((player.2.health as f32) / 10.0 * 100.) as f32), Val::Percent(100.));
            if player.2.health <= 0{
                next_state.set(AppState::MainMenu);
            }
            return;
        }  
    }
}


/// READ ONLY, WRITING TO THIS WILL NOT CHANGE THE ACTUAL DATA
#[derive(Resource)]
pub struct PlayerInfo {
    pub position: Vec3,
    pub rotation: Quat,
    pub forward: Vec3,
}

fn update_player_info(
    player: Query<& Transform, With<Player>>,
    player_children: Query<&Children, With<Player>>,
    transforms: Query<&mut Transform, (Without<Player>, Without<Camera>)>,
    mut player_info: ResMut<PlayerInfo>,
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

// Player Shooting Input
fn shoot_bullet(
    player_info: Res<PlayerInfo>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        
        println!("{}", player_info.forward);
        let mut shooting_point = player_info.position.clone();
        shooting_point.y -= 1.0;
        shooting_point += 0.75*-player_info.forward;
        commands
            .spawn((
                RigidBody::Dynamic,
                Bullet{
                    velocity: player_info.forward,
                },
                Collider::cuboid(0.05, 0.05, 0.25),
                Collidable{kind: CollidableKind::Bullet},
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


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems((
            rotate_player.in_set(OnUpdate(AppState::InGame)),
            rotate_player.in_set(OnUpdate(AppState::InGame)),
            move_player.in_set(OnUpdate(AppState::InGame)),
            update_player_info.in_set(OnUpdate(AppState::InGame)),
            player_damage.in_set(OnUpdate(AppState::InGame)),
            shoot_bullet.in_set(OnUpdate(AppState::InGame)),
            tick_damage_cooldown.in_set(OnUpdate(AppState::InGame)),
        ));
    }
}