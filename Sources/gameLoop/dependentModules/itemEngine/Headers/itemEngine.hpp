//
//  itemEngine.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 10/12/2020.
//

#ifndef itemEngine_hpp
#define itemEngine_hpp

#include <vector>
#include "objectedGraphicsLibrary.hpp"
#include "itemType.hpp"
#include "blockType.hpp"

namespace itemEngine {

struct uniqueItem {
    uniqueItem(const std::string& name, unsigned short stack_size, blockEngine::blockType places);
    std::string name;
    SDL_Texture* texture;
    unsigned short stack_size;
    blockEngine::blockType places;
    ogl::texture text_texture{ogl::top_left};
};

struct item {
    item(itemType item_id, int x, int y);
    itemType item_id;
    int x, y, velocity_x, velocity_y;
    
    void draw() const;
    [[nodiscard]] SDL_Rect getRect() const;
    void update();
    [[nodiscard]] bool colliding() const;
    [[nodiscard]] uniqueItem& getUniqueItem() const;
    unsigned short id;
};

void init();
void close();
void renderItems();
void updateItems();

inline std::vector<uniqueItem> unique_items;
inline std::vector<item> items;

void spawnItem(itemType item_id, int x, int y);

}

#endif /* itemEngine_hpp */
