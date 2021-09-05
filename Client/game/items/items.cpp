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
    sf::VertexArray item_vertex_array(sf::Quads, items.size() * 4);
    int item_index = 0;
    for(ClientItem& item : items) {
        gfx::RectShape rect = resource_pack->getTextureRectangle(item.getType());
        item_vertex_array[item_index    ].texCoords = {0, (float)rect.y};
        item_vertex_array[item_index + 1].texCoords = {(float)rect.w, (float)rect.y};
        item_vertex_array[item_index + 2].texCoords = {(float)rect.w, (float)rect.y + (float)rect.h};
        item_vertex_array[item_index + 3].texCoords = {0, (float)rect.y + (float)rect.h};

        float item_x = (float)item.x / 100 - blocks->view_x + (float)gfx::getWindowWidth() / 2;
        float item_y = (float)item.y / 100 - blocks->view_y + (float)gfx::getWindowHeight() / 2;
        item_vertex_array[item_index    ].position = {item_x, item_y};
        item_vertex_array[item_index + 1].position = {item_x + (float)rect.w * 2, item_y};
        item_vertex_array[item_index + 2].position = {item_x + (float)rect.w * 2, item_y + (float)rect.h * 2};
        item_vertex_array[item_index + 3].position = {item_x, item_y + (float)rect.h * 2};
        //texture.render(2, item.x / 100 - blocks->view_x + gfx::getWindowWidth() / 2, item.y / 100 - blocks->view_y + gfx::getWindowHeight() / 2);
        item_index += 4;
    }
    if(item_index)
        gfx::drawVertices(item_vertex_array, resource_pack->getItemTexture().getSfmlTexture()->getTexture());
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
