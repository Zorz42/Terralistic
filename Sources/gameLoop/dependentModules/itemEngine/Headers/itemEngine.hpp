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
};

struct inventoryItem {
public:
    inventoryItem();
    itemType item_id;
    [[nodiscard]] uniqueItem& getUniqueItem() const;
    void render(int x, int y);
    ogl::texture stack_texture{ogl::top_left};
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
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
inline bool inventory_open = false;

}

#endif /* itemEngine_hpp */
