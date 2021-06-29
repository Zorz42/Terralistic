//
//  inventory.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#ifndef inventory_hpp
#define inventory_hpp

#define INVENTORY_SIZE 20

#include "clientMap.hpp"

class clientInventoryItem {
    unsigned short stack;
public:
    clientInventoryItem() : item_id(itemType::NOTHING), stack(0) {}
    itemType item_id;
    [[nodiscard]] const uniqueItem& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool stack_changed = true;
};

class clientInventory {
    clientInventoryItem mouse_item;
public:
    clientInventoryItem inventory[INVENTORY_SIZE];
    char addItem(itemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    void swapWithMouseItem(clientInventoryItem* item);
    void clearMouseItem();
    clientInventoryItem* getMouseItem();
};

#endif /* inventory_hpp */
