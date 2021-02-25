//
//  gameLoop.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef gameLoop_hpp
#define gameLoop_hpp

#include <string>

#undef main

namespace gameLoop {

int main(const std::string& world_name, bool multiplayer);
inline bool running, online;
inline float frame_length;

}

#endif /* gameLoop_hpp */
