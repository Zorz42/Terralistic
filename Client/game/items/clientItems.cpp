#include "clientItems.hpp"

void ClientItems::init() {
    manager->packet_event.addListener(this);
    item_creation_event.addListener(this);
    entities->entity_deletion_event.addListener(this);
}

void ClientItems::loadTextures() {
    std::vector<gfx::Texture*> item_textures(getNumItemTypes() - 1);

    for(int i = 1; i < getNumItemTypes(); i++) {
        item_textures[i - 1] = new gfx::Texture;
        item_textures[i - 1]->loadFromFile(resource_pack->getFile("/items/" + getItemTypeById(i)->name + ".png"));
    }

    items_atlas.create(item_textures);
    
    for(int i = 1; i < getNumItemTypes(); i++)
        delete item_textures[i - 1];
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
    for(int i = 0; i < entities->getEntities().size(); i++) {
        if(entities->getEntities()[i]->type == EntityType::ITEM) {
            Item* item = (Item*)entities->getEntities()[i];
            gfx::RectShape rect = getItemRectInAtlas(item->getType());
            item_rects.setTextureCoords(item_index, rect);

            int item_x = item->getX() - camera->getX() + gfx::getWindowWidth() / 2;
            int item_y = item->getY() - camera->getY() + gfx::getWindowHeight() / 2;
            item_rects.setRect(item_index, {item_x + rect.w - 8, item_y - rect.h * 2 + 16, rect.h * 2, rect.w * 2});
            item_index++;
        }
    }
    
    if(item_index)
        item_rects.render(item_count, &getItemsAtlasTexture());
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

const gfx::Texture& ClientItems::getItemsAtlasTexture() {
    return items_atlas.getTexture();
}

gfx::RectShape ClientItems::getItemRectInAtlas(ItemType* type) {
    return items_atlas.getRect(type->id - 1);
}
