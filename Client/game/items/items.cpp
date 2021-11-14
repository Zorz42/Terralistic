#include "clientItems.hpp"

void ClientItems::init() {
    manager->packet_event.addListener(this);
    item_creation_event.addListener(this);
    entities->entity_deletion_event.addListener(this);
}

void ClientItems::stop() {
    manager->packet_event.removeListener(this);
    item_creation_event.removeListener(this);
    entities->entity_deletion_event.removeListener(this);
}

void ClientItems::onEvent(ItemCreationEvent& event) {
    item_count++;
    item_rects.resize(item_count);
}

void ClientItems::onEvent(EntityDeletionEvent& event) {
    if(event.entity->type == EntityType::ITEM) {
        item_count--;
        item_rects.resize(item_count);
    }
}

void ClientItems::render() {
    int item_index = 0;
    for(Entity* entity : entities->getEntities()) {
        if(entity->type == EntityType::ITEM) {
            Item* item = (Item*)entity;
            gfx::RectShape rect = resource_pack->getTextureRectangle(item->getType());
            item_rects.setTextureCoords(item_index, rect);

            int item_x = item->getX() - blocks->view_x + gfx::getWindowWidth() / 2;
            int item_y = item->getY() - blocks->view_y + gfx::getWindowHeight() / 2;
            item_rects.setRect(item_index, {item_x, item_y, rect.h * 2, rect.w * 2});
            item_index++;
        }
    }
    
    if(item_index)
        item_rects.render(item_count, &resource_pack->getItemTexture());
}

void ClientItems::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case ServerPacketType::ITEM_CREATION: {
            int x, y;
            int id;
            int type;
            event.packet >> x >> y >> id >> type;
            
            Item* item = new Item(getItemTypeById(type), x, y, id);
            entities->registerEntity(item);
            
            ItemCreationEvent item_event(item);
            item_creation_event.call(item_event);
            
            break;
        }
        default:;
    }
}
