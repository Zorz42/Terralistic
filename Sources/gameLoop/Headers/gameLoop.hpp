//
//  gameLoop.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef gameLoop_h
#define gameLoop_h

#include <string>

#undef main

namespace gameLoop {

int main(const std::string& world_name);
inline bool running, quit = false;

}

#endif /* gameLoop_h */
