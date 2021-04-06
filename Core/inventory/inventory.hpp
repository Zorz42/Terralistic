//
//  inventory.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/12/2020.
//

#ifndef inventory_hpp
#define inventory_hpp

#define INVENTORY_SIZE 20

namespace inventory {

struct inventoryItem {
public:
    inventoryItem();
    explicit inventoryItem(map::itemType item_id) : item_id(item_id), stack(1) {}
    map::itemType item_id;
    map::uniqueItem& getUniqueItem() const;
    void setStack(unsigned short stack_);
    unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    bool stack_changed = true;
private:
    unsigned short stack;
};

struct inventory {
public:
    inventoryItem inventory[INVENTORY_SIZE];
    char addItem(map::itemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    inventoryItem* getSelectedSlot();
    void swapWithMouseItem(inventoryItem* item);
    void clearMouseItem();
    inventoryItem* getMouseItem();
    void clear();
private:
    inventoryItem mouse_item;
};

}

#endif /* inventory_hpp */
