//
//  init.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#define FILENAME init
#define NAMESPACE init
#include "core.hpp"

struct initScript {
    initScript(init::initFunction func) : func(func) {}
    init::initFunction func;
    bool initialized = false;
};

std::vector<initScript>& getInitScripts() {
    static std::vector<initScript> init_functions;
    return init_functions;
}

init::registerInitFunction::registerInitFunction(initFunction function) {
    getInitScripts().emplace_back(initScript(function));
}

void init::initModules() {
    // initialize all modules that need to be. Its mostly texture loading, rendering and preparing shapes, like defining rectangle size.
    bool done_initializing = false;
    while(!done_initializing) {
        done_initializing = true;
        for(initScript& script : getInitScripts())
            if(!script.initialized) {
                if (script.func())
                    script.initialized = true;
                else
                    done_initializing = false;
            }
    }
}
