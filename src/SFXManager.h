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

namespace godot {

class AudioTrack;

class SFXManager : public Node3D {
    GDCLASS(SFXManager, Node3D)

public:
    SFXManager();
    ~SFXManager() override;

    void get_streams_from_array(TypedArray<AudioTrack> streams);
    Ref<AudioTrack> get_audio_track(const String& track_name);

private:
    std::unordered_map<std::string, Ref<AudioTrack>> track_map;
    std::unordered_map<std::string, int32_t> bus_mapping;
    AudioServer *audio_server;

protected:
    static void _bind_methods();
};


    // Soundtrack Resource

class AudioTrack : public Resource {
  GDCLASS(AudioTrack, Resource)

public:
    AudioTrack();
    ~AudioTrack() override;

    enum AudioType {
        AMBIENCE,
        SFX,
        MUSIC
    };

    [[nodiscard]] String get_resource_name() const;
    [[nodiscard]] Ref<AudioStream> get_stream() const;
    [[nodiscard]] AudioType get_audio_type() const;
    [[nodiscard]] bool get_do_amplify() const;
    [[nodiscard]] bool get_do_reverb() const;
    [[nodiscard]] bool get_do_hard_limit() const;

    void set_resource_name(const String &p_name);
    void set_stream(const Ref<AudioStream>& p_stream);
    void set_do_amplify(bool p_value);
    void set_do_reverb(bool p_value);
    void set_do_hard_limit(bool p_value);
    void set_audio_type(AudioType p_type);

private:
    String name;
    Ref<AudioStream> stream;
    AudioType audio_type;

    bool doAmplify{false};
    bool doReverb{false};
    bool doHardLimit{false};

protected:
    static void _bind_methods();
};

} // godot

VARIANT_ENUM_CAST(godot::AudioTrack::AudioType)

#endif //SFXMANAGER_H
