//
// Created by cmh on 11/15/24.
//


#include "SFXManager.h"
#include <godot_cpp/variant/utility_functions.hpp>
#include "Utilities.h"

using namespace godot;
using namespace darkfire;

SFXManager::SFXManager() {
    audio_server = AudioServer::get_singleton();
    for (int i = 0; i < audio_server->get_bus_count(); i++) {
        String name = audio_server->get_bus_name(i);
        int32_t bus_index = audio_server->get_bus_index(name);

        std::string bus_name = Utilities::convert_godot_string(name);
        bus_mapping.insert_or_assign(bus_name, bus_index);
    }
}


void SFXManager::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_streams_from_array", "stream_name"),
        &SFXManager::get_streams_from_array);
    ClassDB::bind_method(D_METHOD("get_audio_track", "track_name"),
        &SFXManager::get_audio_track);
}

void SFXManager::get_streams_from_array(TypedArray<AudioTrack> streams) {
    for (int i = 0; i < streams.size(); ++i) {
        Ref<AudioTrack> stream = streams[i];
        if (stream.is_valid()) {
            auto res_path = Utilities::convert_godot_string(stream->get_path());
            res_path = Utilities::get_substr(res_path, "track");
            Utilities::remove_file_extension(res_path);

            stream->set_track_name(String(res_path.c_str()));
            track_map.insert_or_assign(res_path, stream);
        }
    }
}


void SFXManager::retrieve_audio_track(std::string& track_string_ref) {
    std::string with_track = "track_";
    with_track.append(track_string_ref);

    for (const auto &it : track_map) {
        if (auto it_first = it.first; it_first == with_track) {
            track_string_ref.assign(with_track);
            break;
        }
    }
}


Ref<AudioTrack> SFXManager::get_audio_track(String track_name) {
    std::string track_string = Utilities::convert_godot_string(track_name);
    retrieve_audio_track(track_string);

    return track_map.at(track_string);
}

SFXManager::~SFXManager() = default;