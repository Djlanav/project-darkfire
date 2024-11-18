//
// Created by cmh on 11/15/24.
//


#include "SFXManager.h"
#include <godot_cpp/variant/utility_functions.hpp>
#include "Utilities.h"

using namespace godot;
using namespace darkfire;

SFXManager::SFXManager(): track_map() {
    audio_server = AudioServer::get_singleton();
    for (int i = 0; i < audio_server->get_bus_count(); i++) {
        String name = audio_server->get_bus_name(i);
        int32_t bus_index = audio_server->get_bus_index(name);

        std::string bus_name = Utilities::convert_godot_string(name);
        bus_mapping.insert({bus_name, bus_index});
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
            auto final_name = Utilities::get_substr(res_path, "track");

            track_map.insert({final_name, stream});
        }
    }
}

Ref<AudioTrack> SFXManager::get_audio_track(const String& track_name) {
    const std::string name = Utilities::convert_godot_string(track_name);
    return track_map.at(name);
}

SFXManager::~SFXManager() = default;

// AudioTrack
AudioTrack::AudioTrack() : audio_type(AMBIENCE) {};
AudioTrack::~AudioTrack() = default;

void AudioTrack::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_resource_name"), &AudioTrack::get_name);
    ClassDB::bind_method(D_METHOD("set_resource_name", "p_name"), &AudioTrack::set_name);

    ClassDB::bind_method(D_METHOD("get_stream"), &AudioTrack::get_stream);
    ClassDB::bind_method(D_METHOD("set_stream", "p_stream"), &AudioTrack::set_stream);

    ClassDB::bind_method(D_METHOD("get_do_amplify"), &AudioTrack::get_do_amplify);
    ClassDB::bind_method(D_METHOD("get_do_hard_limit"), &AudioTrack::get_do_hard_limit);
    ClassDB::bind_method(D_METHOD("get_do_reverb"), &AudioTrack::get_do_reverb);

    ClassDB::bind_method(D_METHOD("set_do_reverb", "p_value"), &AudioTrack::set_do_reverb);
    ClassDB::bind_method(D_METHOD("set_do_hard_limit", "p_value"), &AudioTrack::set_do_hard_limit);
    ClassDB::bind_method(D_METHOD("set_do_amplify", "p_value"), &AudioTrack::set_do_amplify);

    ClassDB::bind_method(D_METHOD("set_audio_type", "p_value"), &AudioTrack::set_audio_type);
    ClassDB::bind_method(D_METHOD("get_audio_type"), &AudioTrack::get_audio_type);

    BIND_ENUM_CONSTANT(SFX)
    BIND_ENUM_CONSTANT(MUSIC)
    BIND_ENUM_CONSTANT(AMBIENCE)

    ADD_PROPERTY(PropertyInfo(Variant::OBJECT, "stream"), "set_stream", "get_stream");
    ADD_PROPERTY(PropertyInfo(Variant::INT, "audio_type",PROPERTY_HINT_ENUM,
        "SFX, Music, Ambience"), "set_audio_type", "get_audio_type");

    ADD_PROPERTY(PropertyInfo(Variant::BOOL, "doAmplify"), "set_do_amplify", "get_do_amplify");
    ADD_PROPERTY(PropertyInfo(Variant::BOOL, "doReverb"), "set_do_reverb", "get_do_reverb");
    ADD_PROPERTY(PropertyInfo(Variant::BOOL, "doHardLimit"), "set_do_hard_limit", "get_do_hard_limit");
}


String AudioTrack::get_resource_name() const {
    return name;
}

Ref<AudioStream> AudioTrack::get_stream() const {
    return stream;
}

AudioTrack::AudioType AudioTrack::get_audio_type() const {
    return audio_type;
}

void AudioTrack::set_audio_type(AudioType p_type) {
    audio_type = p_type;
}

bool AudioTrack::get_do_amplify() const {
    return doAmplify;
}

bool AudioTrack::get_do_hard_limit() const {
    return doHardLimit;
}

bool AudioTrack::get_do_reverb() const {
    return doReverb;
}

void AudioTrack::set_do_amplify(bool p_value) {
    doAmplify = p_value;
}

void AudioTrack::set_do_hard_limit(bool p_value) {
    doHardLimit = p_value;
}

void AudioTrack::set_do_reverb(bool p_value) {
    doReverb = p_value;
}


void AudioTrack::set_resource_name(const String &p_name) {
    name = p_name;
}

void AudioTrack::set_stream(const Ref<AudioStream>& p_stream) {
    stream = p_stream;
}