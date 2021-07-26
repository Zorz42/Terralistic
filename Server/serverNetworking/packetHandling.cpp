#include "serverNetworking.hpp"
#include "print.hpp"

void NetworkingManager::onPacket(sf::Packet &packet, PacketType packet_type, Connection &conn) {
    ServerPlayer* curr_player = conn.player;
    switch (packet_type) {
        case PacketType::STARTED_BREAKING: {
            unsigned short x, y;
            packet >> x >> y;
            curr_player->breaking_x = x;
            curr_player->breaking_y = y;
            curr_player->breaking = true;
            break;
        }

        case PacketType::STOPPED_BREAKING: {
            curr_player->breaking = false;
            break;
        }

        case PacketType::RIGHT_CLICK: {
            unsigned short x, y;
            packet >> x >> y;
            players->rightClickEvent(blocks->getBlock(x, y), curr_player);
            break;
        }

        case PacketType::CHUNK: {
            unsigned short x, y;
            packet >> x >> y;
            
            sf::Packet chunk_packet;
            chunk_packet << PacketType::CHUNK << x << y;
            for(int chunk_x = 0; chunk_x < 16; chunk_x++)
                for(int chunk_y = 0; chunk_y < 16; chunk_y++) {
                    Block block = blocks->getBlock((x << 4) + chunk_x, (y << 4) + chunk_y);
                    chunk_packet << (unsigned char)block.getBlockType() << (unsigned char)block.getLiquidType() << (unsigned char)block.getLiquidLevel() << (unsigned char)block.getLightLevel();
                }
            conn.send(chunk_packet);
            break;
        }

        case PacketType::VIEW_SIZE_CHANGE: {
            packet >> curr_player->sight_width >> curr_player->sight_height;
            break;
        }

        case PacketType::PLAYER_MOVEMENT: {
            packet >> curr_player->x >> curr_player->y >> curr_player->flipped;
            
            sf::Packet movement_packet;
            movement_packet << PacketType::PLAYER_MOVEMENT << curr_player->x << curr_player->y << curr_player->flipped << curr_player->id;
            sendToEveryone(movement_packet, &conn);
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            unsigned char pos;
            packet >> pos;
            curr_player->inventory.swapWithMouseItem(&curr_player->inventory.inventory_arr[pos]);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            packet >> curr_player->inventory.selected_slot;
            break;
        }

        case PacketType::CHAT: {
            std::string message;
            packet >> message;
            std::string chat_format = (curr_player->name == "_" ? "Protagonist" : curr_player->name) + ": " + message;
            print::info(chat_format);
            
            sf::Packet chat_packet;
            chat_packet << PacketType::CHAT << chat_format;
            sendToEveryone(chat_packet);
            break;
        }
            
        case PacketType::VIEW_POS_CHANGE: {
            packet >> curr_player->sight_x >> curr_player->sight_y;
            break;
        }

        default:;
    }
}

void NetworkingManager::onEvent(ServerBlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.type;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerBlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS_CHANGE << event.block.getX() << event.block.getY() << event.break_stage;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerLiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.liquid_type << event.liquid_level;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << event.x << event.y << event.id << (unsigned char)event.item_id;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_DELETION << (short)event.item_to_delete.getId();
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemMovementEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_MOVEMENT << event.moved_item.getX() <<  event.moved_item.getY() << event.moved_item.getId();
    sendToEveryone(packet);
}

void NetworkingManager::sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack) {
    sf::Packet packet;
    packet << PacketType::INVENTORY_CHANGE << stack << (unsigned char)type << item.getPosInInventory();
    connection.send(packet);
}

void NetworkingManager::onEvent(ServerInventoryItemStackChangeEvent& event) {
    for(Connection& connection : connections)
        if(connection.player && &connection.player->inventory == event.item.getInventory()) {
            sendInventoryItemPacket(connection, event.item, event.item.getType(), event.stack);
            break;
        }
}

void NetworkingManager::onEvent(ServerInventoryItemTypeChangeEvent& event) {
    for(Connection& connection : connections)
        if(connection.player && &connection.player->inventory == event.item.getInventory()) {
            sendInventoryItemPacket(connection, event.item, event.type, event.item.getStack());
            break;
        }
}

void NetworkingManager::syncLightWithPlayers() {
    for(Connection& connection : connections)
        if(connection.player) {
            int start_x = (int)connection.player->getSightBeginX() - 20, start_y = (int)connection.player->getSightBeginY() - 20, end_x = connection.player->getSightEndX() + 20, end_y = connection.player->getSightEndY() + 20;
            if(start_x < 0)
                start_x = 0;
            if(start_y < 0)
                start_y = 0;
            if(end_x > blocks->getWidth())
                end_x = blocks->getWidth();
            if(end_y > blocks->getHeight())
                end_y = blocks->getHeight();
            
            for(unsigned short y = start_y; y < end_y; y++)
                for(unsigned short x = start_x; x < end_x; x++) {
                    Block curr_block = blocks->getBlock(x, y);
                    if(curr_block.hasLightChanged()) {
                        curr_block.markLightUnchanged();
                        sf::Packet packet;
                        packet << PacketType::LIGHT_CHANGE << curr_block.getX() << curr_block.getY() << curr_block.getLightLevel();
                        connection.send(packet);
                    }
                }
        }
}
