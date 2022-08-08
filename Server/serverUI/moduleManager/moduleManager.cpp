#include "moduleManager.hpp"
#include "console.hpp"

ModuleManager::ModuleManager(): LauncherModule("module_manager"){
    min_width = 1;
    min_height = 1;
}

void ModuleManager::init() {
    for(auto module : module_vector)
        if(*module->getModuleName() == "console"){
            auto console = (Console*)module;
            console->module_manager = this;
        }
}

bool ModuleManager::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::C && getAbsoluteKeyState(gfx::Key::CTRL))
        for(auto module : module_vector)
            if(*module->getModuleName() == "console"){
                module->enabled = !module->enabled;
                return true;
            }
    return false;
}

void ModuleManager::moduleConfig(std::string command) {
    command.erase(0, 14);
    size_t pos = command.find(' ');
    for(auto module : module_vector){
        if(command.substr(0, pos) == *module->getModuleName()){
            command.erase(0, pos + 1);
            pos = command.find(' ');
            auto l_module = (LauncherModule*)module;
            l_module->changeConfig(command.substr(0, pos), command.substr(pos + 1, command.size() - pos - 1));
        }
    }
}