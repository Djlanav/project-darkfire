//
// Created by cmh on 11/15/24.
//

#ifndef SFXMANAGER_H
#define SFXMANAGER_H

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/audio_stream.hpp>
#include <memory>
#include <vector>
#include <unordered_map>

namespace godot {

class RSoundTrack;

class SFXManager : public Node {
    GDCLASS(SFXManager, Node)

public:
    SFXManager();
    ~SFXManager();

    void get_streams_from_array(TypedArray<AudioStream> streams);

private:
    std::unordered_map<std::string, Ref<RSoundTrack>> stream_map;

protected:
    static void _bind_methods();
};


    // Soundtrack Resource
class RSoundTrack : public Resource {
  GDCLASS(RSoundTrack, Resource)

public:
    RSoundTrack();
    ~RSoundTrack();

    String get_name() const;
    void set_name(const String &p_name);

private:
    String name;

protected:
    static void _bind_methods();
};

} // godot

#endif //SFXMANAGER_H
