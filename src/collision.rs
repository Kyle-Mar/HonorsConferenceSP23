use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;

#[derive(Clone, Debug)]
pub enum CollidableKind {
    Player,
    Bullet,
    Enemy,
    Ground,
    Wall,
}

#[derive(Clone, Debug)]
pub struct ObjectInfo{
    pub kind: CollidableKind,
    pub transform: Transform,
}

#[derive(Component, Debug)]
pub struct CollisionTag {
    pub other: ObjectInfo,
    pub this: ObjectInfo,
}
#[derive(Component)]
pub struct Collidable {
    pub kind: CollidableKind,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CollisionInfo {
    pub collisions: HashMap<Entity, Vec<CollisionTag>>,
}

fn collision_test(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut collision_info: ResMut<CollisionInfo>,
    collidables: Query<&Collidable, Without<Sensor>>,
    transforms: Query<&Transform, Without<Sensor>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                if let (Ok(col_a),  Ok(tran_a)) = (collidables.get(*a), transforms.get(*a)){
                    if let (Ok(col_b), Ok(tran_b)) = (collidables.get(*b), transforms.get(*b)){
                        println!("{:?} {:?}", col_a.kind, col_b.kind);
                        if let Some(kv) = collision_info.get_key_value_mut(a){
                            let col_vec = kv.1;
                            col_vec.insert(col_vec.len(), CollisionTag {
                                other: ObjectInfo { kind: col_b.kind.clone(), transform: *tran_b },
                                this: ObjectInfo { kind: col_a.kind.clone(), transform: *tran_a },
                            });
                        } else{
                            let mut col_vec = Vec::<CollisionTag>::new();
                            col_vec.insert(col_vec.len(), CollisionTag {
                                other: ObjectInfo { kind: col_b.kind.clone(), transform: *tran_b },
                                this: ObjectInfo { kind: col_a.kind.clone(), transform: *tran_a },
                            });
                            collision_info.insert(*a, col_vec);
                        }
                        if let Some(kv) = collision_info.collisions.get_key_value_mut(b){
                            let col_vec = kv.1;
                            col_vec.insert(col_vec.len(), CollisionTag {
                                other: ObjectInfo{kind:col_a.kind.clone(), transform: *tran_a},
                                this: ObjectInfo{kind: col_b.kind.clone(), transform: *tran_b},
                            });
                        } else{
                            let mut col_vec = Vec::<CollisionTag>::new();
                            col_vec.insert(col_vec.len(), CollisionTag {
                                other: ObjectInfo{kind:col_a.kind.clone(), transform: *tran_a},
                                this: ObjectInfo{kind: col_b.kind.clone(), transform: *tran_b},
                            });
                            collision_info.insert(*b, col_vec);
                        }
    
                        commands.entity(*a)
                            .insert(CollisionTag {
                                other: ObjectInfo{kind:col_a.kind.clone(), transform: *tran_a},
                                this: ObjectInfo{kind: col_b.kind.clone(), transform: *tran_b},
                            });
    
                        commands.entity(*b)
                            .insert(CollisionTag {
                                other: ObjectInfo{kind:col_a.kind.clone(), transform: *tran_a},
                                this: ObjectInfo{kind: col_b.kind.clone(), transform: *tran_b},
                            });
                            
                    }
                }
            }
            _ => {}
        }
    }
}

fn cleanup_collisions(
    mut commands: Commands,
    mut collision_tags: Query<(Entity, &mut CollisionTag)>,
    mut collision_info: ResMut<CollisionInfo>
){
    collision_info.collisions.clear();
    collision_info.collisions.shrink_to_fit();

    for (entity, _collision_tag) in collision_tags.iter_mut(){
        commands.entity(entity).remove::<CollisionTag>();
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionInfo{
            collisions: HashMap::new()
        })
        .add_system(collision_test.in_base_set(CoreSet::PostUpdateFlush))
        .add_system(cleanup_collisions.in_base_set(CoreSet::PostUpdate));
    }
}