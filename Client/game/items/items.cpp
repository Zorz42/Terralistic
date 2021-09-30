#include "clientItems.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"

void ClientItems::init() {
    manager->packet_event.addListener(this);
}

void ClientItems::renderItems() {
    int item_count = 0;
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::ITEM)
            item_count++;
    gfx::RectArray item_rects(item_count);
    int item_index = 0;
    for(Entity* entity : entities->getEntities()) {
        if(entity->type == EntityType::ITEM) {
            ClientItem* item = (ClientItem*)entity;
            gfx::RectShape rect = resource_pack->getTextureRectangle(item->getType());
            item_rects.setTextureCoords(item_index, rect);

            short item_x = item->x - blocks->view_x + gfx::getWindowWidth() / 2;
            short item_y = item->y - blocks->view_y + gfx::getWindowHeight() / 2;
            item_rects.setRect(item_index, {item_x, item_y, (unsigned short)(rect.h * 2), (unsigned short)(rect.w * 2)});
            item_index++;
        }
    }
    item_rects.setImage(&resource_pack->getItemTexture());
    if(item_index)
        item_rects.render();
}

void ClientItems::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::ITEM_CREATION: {
            int x, y;
            unsigned short id;
            unsigned char type_char;
            event.packet >> x >> y >> id >> type_char;
            ItemType type = (ItemType)type_char;
            
            ClientItem* item = new ClientItem(type, x, y, id);
            entities->registerEntity(item);
            break;
        }
        case PacketType::ITEM_DELETION: {
            unsigned short id;
            event.packet >> id;
            ClientItem* item = (ClientItem*)entities->getEntityById(id);
            entities->removeEntity(item);
            delete item;
        }
        default:;
    }
}
