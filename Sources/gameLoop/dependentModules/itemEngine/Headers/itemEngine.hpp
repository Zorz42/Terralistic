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

enum itemType {NOTHING, STONE};

struct uniqueItem {
    uniqueItem(std::string name);
    std::string name;
    SDL_Texture* texture;
    unsigned short stack_size;
};

struct item {
    item(itemType item_id, int x, int y);
    itemType item_id;
    int x, y, velocity_x, velocity_y;
    
    void draw();
    SDL_Rect getRect();
    void update();
    bool colliding();
    uniqueItem& getUniqueItem();
};

struct inventoryItem {
    inventoryItem(itemType item_id=NOTHING, unsigned int stack=0);
    unsigned short stack;
    itemType item_id;
    uniqueItem& getUniqueItem();
    void render(int x, int y);
};

void init();
void prepare();
void close();
void renderItems();
void updateItems();

inline std::vector<uniqueItem> unique_items;
inline std::vector<item> items;

inline inventoryItem inventory[20];

void spawnItem(itemType item_id, int x, int y);

bool addItemToInventory(itemType id);

}

#endif /* itemEngine_hpp */
