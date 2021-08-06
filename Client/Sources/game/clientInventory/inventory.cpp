#include "clientInventory.hpp"

char ClientInventory::addItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].type == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].type == ItemType::NOTHING) {
            inventory[i].type = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

void ClientInventory::swapWithMouseItem(ClientInventoryItem* item) {
    ClientInventoryItem temp;
    temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

void ClientInventory::clearMouseItem() {
    mouse_item.type = ItemType::NOTHING;
    mouse_item.setStack(0);
}
