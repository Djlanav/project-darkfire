//
// Created by cmh on 11/17/24.
//

#ifndef UTILITIES_H
#define UTILITIES_H

namespace darkfire {
    class Utilities {
    public:
        static std::string convert_godot_string(const godot::String& input);
        static std::string get_substr(const std::string& str, const std::string& substr);
    };
}



#endif //UTILITIES_H
