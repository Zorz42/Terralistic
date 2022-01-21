#include <fstream>
#include <thread>
#include "worldSaver.hpp"
#include "print.hpp"
#include "graphics.hpp"
#include <cstring>
#include <sstream>

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
    std::ostringstream ss;
    ss << file.rdbuf();
    const std::string& s = ss.str();
    std::vector<char> input(s.begin(), s.end());

    int iter = 0;
    while(iter < input.size()) {
        std::string section_name;
        unsigned int section_size;
        while(input[iter])
            section_name.push_back(input[iter++]);
        iter++;
        memcpy(&section_size, &input[iter], sizeof(unsigned int));
        iter += sizeof(unsigned int);

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
        unsigned int size = (unsigned int)i.second.size();
        memcpy(&output[output.size() - 4], &size, sizeof(unsigned int));
        
        output.insert(output.end(), i.second.begin(), i.second.end());
    }

    std::ofstream file(world_path, std::ios::binary);
    std::copy(output.cbegin(), output.cend(), std::ostreambuf_iterator<char>(file));
}

void WorldSaver::update(float frame_length) {
    if(gfx::getTicks() / AUTOSAVE_INTERVAL / 1000 > save_inverval) {
        save_inverval = gfx::getTicks() / AUTOSAVE_INTERVAL / 1000;
        if(autosave_enabled) {
            print::info("Autosaving world...");
            std::thread save_thread(&WorldSaver::save, this);
            save_thread.detach();
        }
    }
}
