#include "clientItems.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"

void ClientItems::renderItems() {
    int item_count = 0;
    for(ClientEntity* entity : entities->getEntities())
        if(entity->type == EntityType::ITEM)
            item_count++;
    sf::VertexArray item_vertex_array(sf::Quads, item_count * 4);
    int item_index = 0;
    for(ClientEntity* entity : entities->getEntities()) {
        if(entity->type == EntityType::ITEM) {
            ClientItem* item = (ClientItem*)entity;
            gfx::RectShape rect = resource_pack->getTextureRectangle(item->getType());
            item_vertex_array[item_index    ].texCoords = {0, (float)rect.y};
            item_vertex_array[item_index + 1].texCoords = {(float)rect.w, (float)rect.y};
            item_vertex_array[item_index + 2].texCoords = {(float)rect.w, (float)rect.y + (float)rect.h};
            item_vertex_array[item_index + 3].texCoords = {0, (float)rect.y + (float)rect.h};

            float item_x = item->x - blocks->view_x + (float)gfx::getWindowWidth() / 2;
            float item_y = item->y - blocks->view_y + (float)gfx::getWindowHeight() / 2;
            item_vertex_array[item_index    ].position = {item_x, item_y};
            item_vertex_array[item_index + 1].position = {item_x + (float)rect.w * 2, item_y};
            item_vertex_array[item_index + 2].position = {item_x + (float)rect.w * 2, item_y + (float)rect.h * 2};
            item_vertex_array[item_index + 3].position = {item_x, item_y + (float)rect.h * 2};
            item_index += 4;
        }
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
            
            ClientItem* item = new ClientItem(type, x, y, id);
            entities->registerEntity(item);
            items.push_back(item);
            break;
        }
        case PacketType::ITEM_DELETION: {
            unsigned short id;
            event.packet >> id;
            ClientItem* item = (ClientItem*)entities->getEntityById(id);
            entities->removeEntity(item);
            items.erase(std::find(items.begin(), items.end(), item));
            delete item;
        }
        default:;
    }
}
