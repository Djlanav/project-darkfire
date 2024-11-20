//
// Created by cmh on 11/15/24.
//

#ifndef SFXMANAGER_H
#define SFXMANAGER_H

#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/audio_stream.hpp>
#include <godot_cpp/classes/audio_server.hpp>
#include <memory>
#include <vector>
#include <unordered_map>
#include "AudioTrack.h"

namespace godot {

class SFXManager : public Node3D {
    GDCLASS(SFXManager, Node3D)

public:
    SFXManager();
    ~SFXManager() override;

    void get_streams_from_array(TypedArray<AudioTrack> streams);

    Ref<AudioTrack> get_audio_track(String track_name);
    void retrieve_audio_track(std::string& track_string_ref);

private:
    std::unordered_map<std::string, Ref<AudioTrack>> track_map{};
    std::unordered_map<std::string, int32_t> bus_mapping{};
    AudioServer *audio_server{};

protected:
    static void _bind_methods();
};

} // godot

#endif //SFXMANAGER_H
