use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioController{
    pub handles: Vec<NamedAudioHandle>
}

#[derive(Clone, Debug)]
pub struct NamedAudioHandle{
    pub name: String,
    pub handle: Handle<AudioSource>,
}

pub trait AddHandle{
    fn add_handle(&mut self, name: &str, handle: Handle<AudioSource>);
}

pub trait GetHandle{
    fn get_handle(&self, name: &str) -> Option<NamedAudioHandle>;
}

impl AddHandle for AudioController{
    fn add_handle(&mut self, name: &str, handle: Handle<AudioSource>){
        if let Some(_handle) = self.get_handle(name){
            println!("Handle with name {} already exists", name);
            return;
        }
        let named_audio_handle = NamedAudioHandle{name: name.to_string(), handle};
        self.handles.insert(self.handles.len(), named_audio_handle);
    }
}

impl GetHandle for AudioController{
    fn get_handle(&self, name: &str) -> Option<NamedAudioHandle> {
        let name_string = name.to_string();
        for handle in self.handles.iter().clone(){
            if handle.name == name_string{
                return Some(handle.clone());
            }
        }
        return None;
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(AudioController{
            handles: Vec::<NamedAudioHandle>::new()
        });
    }
}