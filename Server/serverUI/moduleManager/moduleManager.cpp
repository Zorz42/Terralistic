#include "moduleManager.hpp"
#include "console.hpp"

ModuleManager::ModuleManager(std::string resource_path): LauncherModule("module_manager", std::move(resource_path)){
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
    std::vector<std::string> args;
    size_t pos = command.find(' ');
    while (pos != std::string::npos) {
        args.push_back(command.substr(0, pos));
        command.erase(0, pos + 1);
        pos = command.find(' ');
    }
    args.push_back(command.substr(0, pos));

    for(auto module : module_vector){
        if(args[1] == *module->getModuleName() || args[1] == "all"){
            auto l_module = (LauncherModule*)module;
            if(args.size() == 3 && args[2] == "reload")
                l_module->loadConfig();
            else
                l_module->changeConfig(args[2], args[3]);
        }
    }
}
