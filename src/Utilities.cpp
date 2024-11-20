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

String Utilities::convert_to_godot_string(const std::string &input) {
    String output = input.c_str();
    return output;
}


std::string Utilities::get_substr(const std::string &str, const std::string& substr) {
    size_t find_s = str.find(substr);
    std::string substr_out = str.substr(find_s);

    substr_out.erase(substr_out.find('.'), substr_out.size());
    return substr_out;
}

void Utilities::remove_file_extension(std::string &filename) {
    size_t extension_pos = filename.find('.');
    if (extension_pos != std::string::npos) {
        filename.erase(extension_pos, filename.size());
    }
}
