//
// Created by cmh on 11/17/24.
//

#ifndef UTILITIES_H
#define UTILITIES_H

namespace darkfire {
    class Utilities {
    public:
        static std::string convert_godot_string(const godot::String& input);
        static godot::String convert_to_godot_string(const std::string& input);
        static std::string get_substr(const std::string& str, const std::string& substr);
        static void remove_file_extension(std::string& filename);
    };
}



#endif //UTILITIES_H
