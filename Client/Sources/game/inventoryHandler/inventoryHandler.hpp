#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#define INVENTORY_SIZE 20

#include "properties.hpp"

class clientInventoryItem {
    unsigned short stack;
public:
    clientInventoryItem() : item_id(ItemType::NOTHING), stack(0) {}
    ItemType item_id;
    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool stack_changed = true;
};

class clientInventory {
    clientInventoryItem mouse_item;
public:
    clientInventoryItem inventory[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    bool open = false;
    unsigned char selected_slot = 0;
    void swapWithMouseItem(clientInventoryItem* item);
    void clearMouseItem();
    clientInventoryItem* getMouseItem();
};

class InventoryHandler {
    
};

#endif /* inventoryHandler_hpp */
