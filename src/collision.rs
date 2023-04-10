use bevy::{prelude::*,};
use bevy_rapier3d::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum CollidableKind {
    Player,
    Bullet,
    Enemy,
    Ground,
    Wall,
}

#[derive(Clone, Debug, Copy)]
pub struct ObjectInfo{
    pub entity: Entity,
    pub kind: CollidableKind,
    pub transform: Transform,
}

#[derive(Component, Debug, Clone)]
pub struct CollisionTag {
    pub other: ObjectInfo,
    pub this: ObjectInfo,
}
#[derive(Component)]
pub struct Collidable {
    pub kind: CollidableKind,
}

#[derive(Clone)]
pub struct CollisionStartEvent(pub ObjectInfo, pub ObjectInfo);
#[derive(Clone)]
pub struct CollisionStayEvent(pub ObjectInfo, pub ObjectInfo);
#[derive(Clone)]
pub struct CollisionStopEvent(pub ObjectInfo, pub ObjectInfo);



#[derive(Resource)]
pub struct CollisionEvents {
    pub start_events: Vec<CollisionStartEvent>,
    pub stay_events: Vec<CollisionStayEvent>,
    pub stop_events: Vec<CollisionStopEvent>,
}

fn collision_rapier_handler(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    collidables: Query<&Collidable, Without<Sensor>>,
    transforms: Query<&Transform, Without<Sensor>>,
    mut event_col_start: EventWriter<CollisionStartEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(a, b, _) => {

                let (Ok(col_a), Ok(tran_a)) = (collidables.get(*a), transforms.get(*a)) 
                else{
                    continue;
                };

                let (Ok(col_b), Ok(tran_b)) = (collidables.get(*b), transforms.get(*b))
                else{
                    continue;
                };

                //println!("{:?} {:?}", col_a.kind, col_b.kind);

                let objectinfo_a = ObjectInfo{entity: *a, kind: col_a.kind.clone(), transform: *tran_a};
                let objectinfo_b = ObjectInfo{entity: *b, kind: col_b.kind.clone(), transform: *tran_b};

                event_col_start.send(CollisionStartEvent(objectinfo_a, objectinfo_b));
                //event_col_start.send(CollisionStartEvent(objectinfo_b, objectinfo_a));

                commands.entity(*a)
                    .insert(CollisionTag {
                        other: objectinfo_b,
                        this: objectinfo_a,
                    });

                commands.entity(*b)
                    .insert(CollisionTag {
                        other: objectinfo_a,
                        this: objectinfo_b,
                    });
            }
            _ => {}
        }
    }
}

fn cleanup_collisions(
    mut commands: Commands,
    mut collision_tags: Query<(Entity, &mut CollisionTag)>,
){
    for (entity, _collision_tag) in collision_tags.iter_mut(){
        commands.entity(entity).remove::<CollisionTag>();
    }
}

fn collision_event_accumulator(
    mut collison_start_event: EventReader<CollisionStartEvent>,
    mut collison_stay_event: EventReader<CollisionStayEvent>,
    context: Res<RapierContext>,
    mut events: ResMut<CollisionEvents>,
){
    for event in collison_start_event.iter(){
        let objectinfo_a = event.0;
        let objectinfo_b = event.1;
        
        let contact_pair = context.contact_pair(objectinfo_a.entity, objectinfo_b.entity);
        
        match contact_pair{
            Some(contact_pair) =>{
                if contact_pair.has_any_active_contacts(){
                    events.stay_events.push(CollisionStayEvent(objectinfo_a, objectinfo_b));
                }
            }
            None =>{
                events.stop_events.push(CollisionStopEvent(objectinfo_b, objectinfo_a));
            }
        }
    }

    for event in collison_stay_event.iter(){
        let objectinfo_a = event.0;
        let objectinfo_b = event.1;
        
        let contact_pair = context.contact_pair(objectinfo_a.entity, objectinfo_b.entity);
        
        match contact_pair{
            Some(contact_pair) =>{
                if contact_pair.has_any_active_contacts(){
                    events.stay_events.push(CollisionStayEvent(objectinfo_a, objectinfo_b));
                }
            }
            None =>{
                events.stop_events.push(CollisionStopEvent(objectinfo_a, objectinfo_b));
            }
        }
    }
}

fn collision_event_propegator(
    mut stay_event_writer: EventWriter<CollisionStayEvent>,
    mut stop_event_writer: EventWriter<CollisionStopEvent>,
    mut collision_events: ResMut<CollisionEvents>,
){
    //println!("{}", collision_events.stay_events.len());
    for event in collision_events.stay_events.iter(){
        stay_event_writer.send(event.clone());
    }
    for event in collision_events.stop_events.iter(){
        stop_event_writer.send(event.clone());
    }
    collision_events.stay_events.clear();
    collision_events.stay_events.shrink_to_fit();
    collision_events.stop_events.clear();
    collision_events.stop_events.shrink_to_fit();
}




pub struct CollisionPlugin;

impl Plugin for CollisionPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(CollisionEvents{
            start_events: Vec::new(),
            stay_events: Vec::new(),
            stop_events: Vec::new(),
        })
        .add_system(collision_rapier_handler.in_base_set(CoreSet::PostUpdateFlush))
        .add_event::<CollisionStartEvent>()
        .add_event::<CollisionStayEvent>()
        .add_event::<CollisionStopEvent>()
        .add_system(collision_event_accumulator)
        .add_system(collision_event_propegator.after(collision_event_accumulator))
        .add_system(cleanup_collisions.in_base_set(CoreSet::PostUpdate));
    }
}