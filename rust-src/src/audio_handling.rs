use std::collections::HashMap;
use std::ops::Deref;
use godot::classes::AudioStream;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct SFXManager {
    bus_mapping: HashMap<String, i64>,
    track_mapping: HashMap<String, Gd<AudioTrack>>,
    base: Base<Node3D>
}

#[godot_api]
impl SFXManager {
    #[func]
    fn get_streams_from_array(&mut self, track_array: Array<Gd<AudioTrack>>) {
        for mut track in track_array.iter_shared() {
            let mut res_path: String = track.get_path().into();
            let mut track_bind = track.bind_mut();
            let find_pos = res_path.find("_").unwrap();

            res_path = res_path[find_pos..].to_owned();
            track_bind.track_name = GString::from(res_path.clone());

            drop(track_bind);
            self.track_mapping.insert(res_path, track);
        }
    }

    #[func]
    fn retrieve_audio_track(&self, track_name: GString) -> Gd<AudioTrack> {
        let track_name: String = track_name.into();
        self.track_mapping.get(&track_name).unwrap().clone()
    }
}

#[godot_api]
impl INode3D for SFXManager {
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("[NATIVE RUST API] Initializing SFXManager...");
        let sfx_manager = SFXManager {
            track_mapping: HashMap::new(),
            bus_mapping: HashMap::new(),
            base
        };

        godot_print!("[NATIVE RUST API] SFXManager initialized!");
        sfx_manager
    }
}

#[derive(GodotClass)]
#[class(base=Resource)]
struct AudioTrack {
    track_name: GString,
    #[export]
    stream: Option<Gd<AudioStream>>,
    #[export]
    amplify: bool,
    #[export]
    reverb: bool,
    #[export]
    hard_limit: bool,
    base: Base<Resource>
}

#[godot_api]
impl IResource for AudioTrack {
    fn init(base: Base<Resource>) -> Self {
        Self {
            track_name: GString::default(),
            stream: None,
            amplify: false,
            reverb: false,
            hard_limit: false,
            base
        }
    }
}