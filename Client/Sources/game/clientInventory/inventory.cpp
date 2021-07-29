#include "clientInventory.hpp"

char ClientInventory::addItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == id) {
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory[i].item_id == ItemType::NOTHING) {
            inventory[i].item_id = id;
            quantity -= inventory[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

void ClientInventory::swapWithMouseItem(ClientInventoryItem* item) {
    ClientInventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

void ClientInventory::clearMouseItem() {
    mouse_item.item_id = ItemType::NOTHING;
    mouse_item.setStack(0);
}
