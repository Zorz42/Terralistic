//
//  inventory.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#ifndef inventory_hpp
#define inventory_hpp

#include "itemEngine.hpp"

#define INVENTORY_SIZE 20

namespace inventory {

struct inventoryItem {
public:
    inventoryItem();
    explicit inventoryItem(itemEngine::itemType item_id) : item_id(item_id), stack(1) {}
    itemEngine::itemType item_id;
    [[nodiscard]] itemEngine::uniqueItem& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    inventoryItem& operator=(const inventoryItem& item);
    bool stack_changed = true;
private:
    unsigned short stack;
};

struct inventory {
public:
    inventoryItem inventory[INVENTORY_SIZE];
    char addItem(itemEngine::itemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    inventoryItem* getSelectedSlot();
    inventoryItem mouse_item;
private:
};

}

#endif /* inventory_hpp */
