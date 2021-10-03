#include "serverNetworking.hpp"
#include "print.hpp"

/*void ServerNetworkingManager::onPacket(sf::Packet &packet, PacketType packet_type, Connection &conn) {
    switch (packet_type) {
        case PacketType::STARTED_BREAKING: {
            unsigned short x, y;
            packet >> x >> y;
            conn.player->breaking_x = x;
            conn.player->breaking_y = y;
            conn.player->breaking = true;
            break;
        }

        case PacketType::STOPPED_BREAKING: {
            conn.player->breaking = false;
            break;
        }

        case PacketType::RIGHT_CLICK: {
            unsigned short x, y;
            packet >> x >> y;
            players->rightClickEvent(conn.player, x, y);
            break;
        }

        case PacketType::PLAYER_VELOCITY: {
            float velocity_x, velocity_y;
            packet >> velocity_x >> velocity_y;
            entities->setVelocityX(conn.player, velocity_x);
            entities->setVelocityY(conn.player, velocity_y);
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            unsigned char pos;
            packet >> pos;
            conn.player->inventory.swapWithMouseItem(pos);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            packet >> conn.player->inventory.selected_slot;
            break;
        }

        case PacketType::CHAT: {
            std::string message;
            packet >> message;
            std::string chat_format = (conn.player->name == "_" ? "Protagonist" : conn.player->name) + ": " + message;
            print::info(chat_format);
            if(message.at(0) != '/') {
                sf::Packet chat_packet;
                chat_packet << PacketType::CHAT << chat_format;
                sendToEveryone(chat_packet);
            }else{
                commands.startCommand(message, conn.player->name);
            }
            break;
        }
            
        case PacketType::CRAFT: {
            unsigned char craft_index;
            packet >> craft_index;
            const Recipe* recipe_crafted = conn.player->inventory.getAvailableRecipes()[(int)craft_index];
            conn.player->inventory.addItem(recipe_crafted->result_type, recipe_crafted->result_stack);
            
            for(auto ingredient : recipe_crafted->ingredients)
                conn.player->inventory.removeItem(ingredient.first, ingredient.second);
        }
            
        case PacketType::PLAYER_MOVING_TYPE: {
            unsigned char moving_type;
            packet >> moving_type;
            conn.player->moving_type = (MovingType)moving_type;
            sf::Packet moving_packet;
            moving_packet << PacketType::PLAYER_MOVING_TYPE << moving_type << conn.player->id;
            sendToEveryone(moving_packet);
        }
        
        case PacketType::PLAYER_JUMPED: {
            sf::Packet jumped_packet;
            jumped_packet << PacketType::PLAYER_JUMPED << conn.player->id;
            sendToEveryone(jumped_packet);
        }
            
        default:;
    }
}*/

/*void ServerNetworkingManager::onEvent(BlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK << event.x << event.y << (unsigned char)blocks->getBlockType(event.x, event.y);
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(BlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS << event.x << event.y << blocks->getBreakProgress(event.x, event.y);
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(LiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID << event.x << event.y << (unsigned char)liquids->getLiquidType(event.x, event.y) << liquids->getLiquidLevel(event.x, event.y);
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(ItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << (int)event.item->getX() << (int)event.item->getY() << event.item->id << (unsigned char)event.item->getType();
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(EntityDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_DELETION << (unsigned short)event.entity->id;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(EntityVelocityChangeEvent& event) {
    Connection* exclusion = nullptr;
    for(Connection& conn : connections)
        if(conn.player->id == event.entity->id) {
            exclusion = &conn;
            break;
        }
    
    sf::Packet packet;
    packet << PacketType::ENTITY_VELOCITY << event.entity->getVelocityX() <<  event.entity->getVelocityY() << event.entity->id;
    sendToEveryone(packet, exclusion);
}

void ServerNetworkingManager::syncEntityPositions() {
    for(Entity* entity : entities->getEntities()) {
        sf::Packet packet;
        packet << PacketType::ENTITY_POSITION << entity->getX() << entity->getY() << entity->id;
        sendToEveryone(packet);
    }
}

void ServerNetworkingManager::onEvent(EntityPositionChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_POSITION << event.entity->getX() << event.entity->getY() << event.entity->id;
    sendToEveryone(packet);
}

void Connection::onEvent(InventoryItemChangeEvent& event) {
    ItemStack item = player->inventory.getItem(event.item_pos);
    sf::Packet packet;
    packet << PacketType::INVENTORY << item.stack << (unsigned char)item.type << (short)event.item_pos;
    send(packet);
}*/
