//
// Created by cmh on 11/15/24.
//


#include "SFXManager.h"
#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;

SFXManager::SFXManager(): stream_map() {
}

void SFXManager::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_streams_from_array", "stream_name"),
        &SFXManager::get_streams_from_array);
}

void SFXManager::get_streams_from_array(TypedArray<RSoundTrack> streams) {
    for (int i = 0; i < streams.size(); ++i) {
        Ref<RSoundTrack> stream = streams[i];
        if (stream.is_valid()) {
            UtilityFunctions::print("Stream path: ", stream->get_path());
            std::string path = stream->get_path().utf8().get_data();

            stream_map.insert({path, stream});
        }
    }
}

SFXManager::~SFXManager() = default;

// RSoundTrack
void RSoundTrack::_bind_methods() {
    ClassDB::bind_method(D_METHOD("get_name"), &RSoundTrack::get_name);
    ClassDB::bind_method(D_METHOD("set_name", "p_name"), &RSoundTrack::set_name);

    ClassDB::bind_method(D_METHOD("get_stream"), &RSoundTrack::get_stream);
    ClassDB::bind_method(D_METHOD("set_stream", "p_stream"), &RSoundTrack::set_stream);

    ADD_PROPERTY(PropertyInfo(Variant::STRING, "name"), "set_name", "get_name");
    ADD_PROPERTY(PropertyInfo(Variant::OBJECT, "stream"), "set_stream", "get_stream");
}

RSoundTrack::RSoundTrack() = default;

RSoundTrack::~RSoundTrack() = default;



String RSoundTrack::get_name() const {
    return name;
}

void RSoundTrack::set_name(const String &p_name) {
    name = p_name;
}

Ref<AudioStream> RSoundTrack::get_stream() const {
    return stream;
}

void RSoundTrack::set_stream(Ref<AudioStream> p_stream) {
    stream = p_stream;
}