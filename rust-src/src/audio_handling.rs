use std::collections::HashMap;
use godot::classes::{AudioServer, AudioStream};
use godot::obj::NewGd;
use godot::prelude::*;
use rand::Rng;
use crate::utils;

#[derive(GodotClass)]
#[class(init, base=Resource)]
struct AudioTrack {
    #[var]
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

#[derive(GodotClass)]
#[class(base=Node3D)]
struct SFXManager {
    bus_mapping: HashMap<GString, i64>,
    track_mapping: HashMap<String, Gd<AudioTrack>>,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for SFXManager {
    fn init(base: Base<Node3D>) -> Self {
        let sfx_manager = SFXManager {
            track_mapping: HashMap::new(),
            bus_mapping: HashMap::new(),
            base
        };

        sfx_manager
    }
}

#[godot_api]
impl SFXManager {
    #[func]
    fn retrieve_audio_track(&self, track_name: String) -> Gd<AudioTrack> {
        let track = match self.track_mapping.get(&track_name) {
            Some(track) => track,
            None => panic!("Could not find audio track {}", track_name)
        };

        track.clone()
    }

    #[func]
    fn retrieve_random_ambience(&self) -> Option<Gd<AudioTrack>> {
        let rand_num = rand::thread_rng().gen_range(0..self.track_mapping.len());
        let mut index = 0;

        for key in self.track_mapping.keys() {
            if index == rand_num {
                let track = self.track_mapping.get(key).unwrap();
                return Some(track.clone());
            }

            index += 1;
        }

        None
    }

    #[func]
    fn get_tracks_from_array(&mut self, track_array: Array<Gd<AudioTrack>>) {
        for mut track in track_array.iter_shared() {
            let mut res_path: String = track.get_path().into();
            let find_pos = res_path.find("_").unwrap();

            res_path = res_path[find_pos + 1..].to_owned();
            utils::remove_file_extension(&mut res_path);

            {
                let mut track_bind = track.bind_mut();
                track_bind.set_track_name(GString::from(res_path.clone()));
            }

            self.track_mapping.insert(res_path, track.clone());
        }
    }

    #[func]
    fn get_bus_mapping(&mut self) {
        let audio_server = AudioServer::singleton();
        let bus_count = audio_server.get_bus_count();

        for  bus in 0..bus_count {
            let bus_name = audio_server.get_bus_name(bus);
            let bus_index = audio_server.get_bus_index(bus_name.arg()) as i64;

            self.bus_mapping.insert(bus_name, bus_index);
        }
    }

    #[func]
    fn debug_get_all_track_mappings(&self) {
        for map_pair in &self.track_mapping {
            let track = map_pair.1.bind();
            godot_print!("Track: {}", track.get_track_name());

            drop(track);
        }
    }
}