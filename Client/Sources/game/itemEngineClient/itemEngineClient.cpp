//
//  itemRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#include "core.hpp"

#include "itemEngineClient.hpp"
#include "playerHandler.hpp"

static itemEngineClient::uniqueRenderItem* unique_render_items;

INIT_SCRIPT
    INIT_ASSERT(!itemEngine::unique_items.empty());
    unique_render_items = new itemEngineClient::uniqueRenderItem[itemEngine::unique_items.size()];
    for(int i = 0; i < itemEngine::unique_items.size(); i++) {
        unique_render_items[i].texture.setTexture(itemEngine::unique_items[i].name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + itemEngine::unique_items[i].name + ".png"));
        unique_render_items[i].texture.scale = 2;
        unique_render_items[i].text_texture.setTexture(gfx::renderText(itemEngine::unique_items[i].name, {255, 255, 255}));
        unique_render_items[i].text_texture.scale = 2;
    }
INIT_SCRIPT_END

itemEngineClient::uniqueRenderItem& itemEngineClient::getUniqueRenderItem(unsigned short id) {
    return unique_render_items[id];
}


void itemEngineClient::module::init() {
    listening_to = {packets::ITEM_CREATION, packets::ITEM_DELETION, packets::ITEM_MOVEMENT};
}

void itemEngineClient::module::render() {
    for(itemEngine::item& i : itemEngine::items)
        gfx::render(unique_render_items[i.getItemId()].texture, short(i.x / 100 - playerHandler::view_x + gfx::getWindowWidth() / 2), short(i.y / 100 - playerHandler::view_y + gfx::getWindowHeight() / 2));
}

void itemEngineClient::module::stop() {
    itemEngine::close();
}

void itemEngineClient::module::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::ITEM_CREATION: {
            auto type = (itemEngine::itemType)packet.getChar();
            unsigned short id = packet.getUShort();
            int y = packet.getInt(), x = packet.getInt();
            itemEngine::spawnItem(type, x, y, id);
            break;
        }
        case packets::ITEM_DELETION: {
            unsigned short id = packet.getUShort();
            for(auto i = itemEngine::items.begin(); i != itemEngine::items.end(); i++)
                if(i->getId() == id) {
                    itemEngine::items.erase(i);
                    break;
                }
            break;
        }
        case packets::ITEM_MOVEMENT: {
            itemEngine::item* item = itemEngine::getItemById(packet.getUShort());
            item->y = packet.getInt();
            item->x = packet.getInt();
            break;
        }
        default:;
    }
}
