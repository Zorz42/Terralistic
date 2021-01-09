//
//  inventory.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#ifndef inventory_hpp
#define inventory_hpp


#include "itemEngine.hpp"

namespace inventory {

struct inventoryItem {
public:
    inventoryItem();
    inventoryItem(itemEngine::itemType item_id) : item_id(item_id) {}
    itemEngine::itemType item_id;
    [[nodiscard]] itemEngine::uniqueItem& getUniqueItem() const;
    void render(int x, int y);
    ogl::texture stack_texture{ogl::top_left};
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    void operator=(inventoryItem& item);
private:
    unsigned short stack;
};

void init();
void prepare();
void render();
bool handleEvents(SDL_Event& event);

bool addItemToInventory(itemEngine::itemType id, int quantity);
void selectSlot(char slot);

inline inventoryItem* hovered = nullptr;
inline bool inventory_open = false;
inline inventoryItem *selected_item = nullptr, mouse_item{itemEngine::NOTHING};
inline char selected_slot = 0;
inline inventoryItem player_inventory[20];

}

#endif /* inventory_hpp */
