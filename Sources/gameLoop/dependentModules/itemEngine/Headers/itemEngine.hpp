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
#include "objectedGraphicsLibrary.hpp"

namespace itemEngine {

enum itemType {NOTHING, STONE, DIRT, STONE_BLOCK};

struct uniqueItem {
    uniqueItem(std::string name, unsigned short stack_size);
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
public:
    inventoryItem();
    itemType item_id;
    uniqueItem& getUniqueItem();
    void render(int x, int y);
    ogl::texture stack_texture{(ogl::top_left)};
    void setStack(unsigned short stack_);
    unsigned short getStack();
    unsigned short increaseStack(int stack_);
private:
    unsigned short stack;
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
bool addItemToInventory(itemType id, int quantity);

void selectSlot(char slot);

inline inventoryItem* selected_item = nullptr;
inline char selected_slot = 0;

bool handleEvents(SDL_Event& event);

}

#endif /* itemEngine_hpp */
