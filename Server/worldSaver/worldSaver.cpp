#include <fstream>
#include <thread>
#include "worldSaver.hpp"
#include "print.hpp"
#include "graphics.hpp"
#include <cstring>

#define AUTOSAVE_INTERVAL (5 * 60)

void WorldSaver::setSectionData(const std::string& name, const std::vector<char>& data) {
    sections[name] = data;
}

const std::vector<char>& WorldSaver::getSectionData(const std::string& name) {
    return sections[name];
}

void WorldSaver::load() {
    sections.clear();
    
    std::ifstream file(world_path, std::ios::binary);
    file.unsetf(std::ios::skipws);
    std::streampos file_size;
    file.seekg(0, std::ios::end);
    file_size = file.tellg();
    file.seekg(0, std::ios::beg);
    std::vector<char> input;
    input.reserve(file_size);

    input.insert(input.begin(), std::istream_iterator<char>(file), std::istream_iterator<char>());
    
    int iter = 0;
    while(iter < input.size()) {
        std::string section_name;
        int section_size;
        while(input[iter] != 0)
            section_name.push_back(input[iter++]);
        iter++;
        memcpy(&section_size, &input[iter], sizeof(int));
        iter += sizeof(int);

        sections[section_name] = std::vector<char>(input.begin() + iter, input.begin() + iter + section_size);
        iter += section_size;
    }
    
    WorldLoadEvent event;
    world_load_event.call(event);
}

void WorldSaver::save() {
    WorldSaveEvent event;
    world_save_event.call(event);
    
    std::vector<char> output;
    
    for(auto i : sections) {
        output.insert(output.end(), i.first.begin(), i.first.end());
        output.push_back(0);
        
        output.insert(output.end(), {0, 0, 0, 0});
        int size = (int)i.second.size();
        memcpy(&output[output.size() - 4], &size, sizeof(int));
        
        output.insert(output.end(), i.second.begin(), i.second.end());
    }
    
    std::ofstream file(world_path);
    std::copy(output.cbegin(), output.cend(), std::ostreambuf_iterator<char>(file));
}

void WorldSaver::update(float frame_length) {
    if(gfx::getTicks() / AUTOSAVE_INTERVAL / 1000 > save_inverval) {
        print::info("Autosaving world...");
        save_inverval = gfx::getTicks() / AUTOSAVE_INTERVAL / 1000;
        std::thread save_thread(&WorldSaver::save, this);
        save_thread.detach();
    }
}
