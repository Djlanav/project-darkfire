//
// Created by cmh on 11/18/24.
//

#include <godot_cpp/variant/utility_functions.hpp>
#include "AudioTrack.h"

using namespace godot;

AudioTrack::AudioTrack() : audio_type(AMBIENCE) {};
AudioTrack::~AudioTrack() = default;

void AudioTrack::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_track_name"), &AudioTrack::get_track_name);
    ClassDB::bind_method(D_METHOD("set_track_name", "p_name"), &AudioTrack::set_track_name);

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


String AudioTrack::get_track_name() const {
    return name;
}

Ref<AudioStream> AudioTrack::get_stream() const {
    return stream;
}

AudioType AudioTrack::get_audio_type() const {
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


void AudioTrack::set_track_name(const String &p_name) {
    name = p_name;
}

void AudioTrack::set_stream(const Ref<AudioStream>& p_stream) {
    stream = p_stream;
}