//
// Created by cmh on 11/18/24.
//

#ifndef AUDIOTRACK_H
#define AUDIOTRACK_H

#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/audio_stream.hpp>
#include <godot_cpp/classes/audio_server.hpp>

enum AudioType {
    AMBIENCE,
    SFX,
    MUSIC
};

namespace godot {
    class AudioTrack : public Resource {
    GDCLASS(AudioTrack, Resource)

  public:
    AudioTrack();
    ~AudioTrack() override;

    [[nodiscard]] String get_track_name() const;
    [[nodiscard]] Ref<AudioStream> get_stream() const;
    [[nodiscard]] AudioType get_audio_type() const;
    [[nodiscard]] bool get_do_amplify() const;
    [[nodiscard]] bool get_do_reverb() const;
    [[nodiscard]] bool get_do_hard_limit() const;

    void set_track_name(const String &p_name);
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

}

VARIANT_ENUM_CAST(AudioType)

#endif //AUDIOTRACK_H
