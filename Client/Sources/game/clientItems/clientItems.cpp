#include <random>
#include <cassert>
#include "clientItems.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"

ClientItem* ClientItems::getItemById(unsigned short id) {
    for(ClientItem& i : items)
        if(i.getId() == id)
            return &i;
    assert(false);
    return nullptr;
}

const ItemInfo& ClientItem::getUniqueItem() const {
    return ::getItemInfo(item_type);
}

void ClientItems::renderItems() {
    for(ClientItem& item : items) {
        const gfx::Image& texture = resource_pack->getItemTexture(item.getType());
        texture.render(2, item.x / 100 - blocks->view_x + gfx::getWindowWidth() / 2, item.y / 100 - blocks->view_y + gfx::getWindowHeight() / 2);
    }
}

void ClientItems::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::ITEM_CREATION: {
            int x, y;
            unsigned short id;
            unsigned char type_char;
            event.packet >> x >> y >> id >> type_char;
            ItemType type = (ItemType)type_char;
            
            items.emplace_back(ClientItem(type, x, y, id));
            break;
        }
        case PacketType::ITEM_DELETION: {
            unsigned short id;
            event.packet >> id;
            for(auto i = items.begin(); i != items.end(); i++)
                if(i->getId() == id) {
                    items.erase(i);
                    break;
                }
            break;
        }
        case PacketType::ITEM_MOVEMENT: {
            unsigned short id;
            int x, y;
            event.packet >> x >> y >> id;
            
            ClientItem* item = getItemById(id);
            item->x = x;
            item->y = y;
            break;
        }
        default:;
    }
}
