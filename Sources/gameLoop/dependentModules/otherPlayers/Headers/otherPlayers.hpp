//
//  otherPlayers.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 16/01/2021.
//

#ifndef otherPlayers_hpp
#define otherPlayers_hpp

#include <vector>

namespace players {

void prepare();
void render();

struct player {
    unsigned short id{0};
    int x{0}, y{0};
    bool flipped = false;
};

inline std::vector<player> players;

}

#endif /* otherPlayers_hpp */
