//
// Created by cmh on 11/15/24.
//


#include "SFXManager.h"

using namespace godot;

SFXManager::SFXManager() {

}

void SFXManager::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_streams_from_array", "stream_name"),
        &SFXManager::get_streams_from_array);
}

void SFXManager::get_streams_from_array(TypedArray<AudioStream> streams) {
    for (int i = 0; i < streams.size(); ++i) {
        AudioStream stream = static_cast<AudioStream>(streams[i]);
    }
}

SFXManager::~SFXManager() {

}

// RSoundTrack
void RSoundTrack::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_name"), &RSoundTrack::get_name);
    ClassDB::bind_method(D_METHOD("set_name", "p_name"), &RSoundTrack::set_name);

    ADD_PROPERTY(PropertyInfo(Variant::STRING, "name"), "set_name", "get_name");
}

RSoundTrack::RSoundTrack() {

}

RSoundTrack::~RSoundTrack() {

}



String RSoundTrack::get_name() const {
    return name;
}

void RSoundTrack::set_name(const String &p_name) {
    name = p_name;
}