//
//  itemRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#include "itemRenderer.hpp"
//#include "playerHandler.hpp"/

static itemRenderer::uniqueRenderItem* unique_render_items;

itemRenderer::uniqueRenderItem& itemRenderer::getUniqueRenderItem(map::itemType id) {
    return unique_render_items[(int)id];
}

void itemRenderer::loadItems() {
    unique_render_items = new itemRenderer::uniqueRenderItem[map::unique_items.size()];
    for(int i = 0; i < map::unique_items.size(); i++) {
        unique_render_items[i].texture.setTexture(map::unique_items[i].name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + map::unique_items[i].name + ".png"));
        unique_render_items[i].texture.scale = 2;
        unique_render_items[i].text_texture.setTexture(gfx::renderText(map::unique_items[i].name, {255, 255, 255}));
        unique_render_items[i].text_texture.scale = 2;
    }
}

void itemRenderer::init() {
    listening_to = {packets::ITEM_CREATION, packets::ITEM_DELETION, packets::ITEM_MOVEMENT};
}

void itemRenderer::render() {
    for(map::item& i : scene->world_map.items)
        gfx::render(unique_render_items[(int)i.getItemId()].texture, short(i.x / 100 - playerHandler::view_x + gfx::getWindowWidth() / 2), short(i.y / 100 - playerHandler::view_y + gfx::getWindowHeight() / 2));
}

void itemRenderer::stop() {
    
}

void itemRenderer::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::ITEM_CREATION: {
            auto type = (map::itemType)packet.getChar();
            unsigned short id = packet.getUShort();
            int y = packet.getInt(), x = packet.getInt();
            scene->world_map.spawnItem(type, x, y, id);
            break;
        }
        case packets::ITEM_DELETION: {
            unsigned short id = packet.getUShort();
            for(auto i = scene->world_map.items.begin(); i != scene->world_map.items.end(); i++)
                if(i->getId() == id) {
                    scene->world_map.items.erase(i);
                    break;
                }
            break;
        }
        case packets::ITEM_MOVEMENT: {
            map::item* item = scene->world_map.getItemById(packet.getUShort());
            item->y = packet.getInt();
            item->x = packet.getInt();
            break;
        }
        default:;
    }
}
