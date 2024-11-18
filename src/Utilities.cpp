//
// Created by cmh on 11/17/24.
//

#include <string>
#include <godot_cpp/classes/node.hpp>
#include "Utilities.h"

using namespace darkfire;
using namespace godot;

std::string Utilities::convert_godot_string(const String &input) {
    std::string output = input.utf8().get_data();
    return output;
}

std::string Utilities::get_substr(const std::string &str, const std::string& substr) {
    size_t find_s = str.find(substr);
    return  str.substr(find_s);
}
