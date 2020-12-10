//
//  itemEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#ifndef itemEngine_hpp
#define itemEngine_hpp

#include <SDL2/SDL.h>

#include <string>
#include <vector>

namespace itemEngine {

enum itemType {STONE};

class uniqueItem {
    
};

class item {
    itemType item_id;
    int x, y;
};

void init();
void prepare();
void close();
void renderItems();
void updateItems();

inline std::vector<uniqueItem> unique_items;
inline std::vector<item> items;

}

#endif /* itemEngine_hpp */
