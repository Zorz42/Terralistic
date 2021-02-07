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
    inventoryItem();
    explicit inventoryItem(itemEngine::itemType item_id) : item_id(item_id), stack(1) {}
    itemEngine::itemType item_id;
    [[nodiscard]] itemEngine::uniqueItem& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    inventoryItem& operator=(const inventoryItem& item);
    unsigned short stack;
};

struct inventory {
    inventoryItem inventory[20];
    bool addItem(itemEngine::itemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    inventoryItem mouse_item{itemEngine::NOTHING};
};

}

#endif /* inventory_hpp */
