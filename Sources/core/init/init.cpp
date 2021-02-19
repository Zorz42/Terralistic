//
//  init.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#include <vector>
#include "init.hpp"

std::vector<init::initFunction>& getInitFunctions() {
    static std::vector<init::initFunction> init_functions;
    return init_functions;
}

init::registerInitFunction::registerInitFunction(initFunction function) {
    getInitFunctions().push_back(function);
}

void init::initModules() {
    // initialize all modules, that need to be. Its mostly texture loading, rendering and preparing shapes, like defining rectangle size.
    for(init::initFunction func : getInitFunctions())
        func();
}
