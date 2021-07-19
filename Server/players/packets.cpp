//
//  packets.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 04/05/2021.
//

#ifdef _WIN32
#include <winsock2.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "print.hpp"
#include "players.hpp"

void players::onEvent(ServerPacketEvent& event) {
    player* curr_player = &event.sender;
    switch (event.packet_type) {
        case PacketType::STARTED_BREAKING: {
            unsigned short x, y;
            event.packet >> x >> y;
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
            event.packet >> x >> y;
            rightClickEvent(parent_blocks->getBlock(x, y), curr_player);
            break;
        }

        case PacketType::CHUNK: {
            unsigned short x, y;
            event.packet >> x >> y;
            
            sf::Packet chunk_packet;
            chunk_packet << PacketType::CHUNK << x << y;
            for(int chunk_x = 0; chunk_x < 16; chunk_x++)
                for(int chunk_y = 0; chunk_y < 16; chunk_y++) {
                    Block block = parent_blocks->getBlock((x << 4) + chunk_x, (y << 4) + chunk_y);
                    chunk_packet << (unsigned char)block.getBlockType() << (unsigned char)block.getLiquidType() << (unsigned char)block.getLiquidLevel() << (unsigned char)block.getLightLevel();
                }
            event.sender.socket->send(chunk_packet);
            break;
        }

        case PacketType::VIEW_SIZE_CHANGE: {
            event.packet >> curr_player->sight_width >> curr_player->sight_height;
            break;
        }

        case PacketType::PLAYER_MOVEMENT: {
            event.packet >> curr_player->x >> curr_player->y >> curr_player->flipped;
            
            sf::Packet movement_packet;
            movement_packet << PacketType::PLAYER_MOVEMENT << curr_player->x << curr_player->y << (char)curr_player->flipped << curr_player->id;
            sendToEveryone(movement_packet, curr_player);
            break;
        }

        case PacketType::DISCONNECT: {
            print::info(curr_player->name + " (" + curr_player->socket->getRemoteAddress().toString() + ") disconnected (" + std::to_string(online_players.size() - 1) + " players online)");
            delete curr_player->socket;
            for(auto i = online_players.begin(); i != online_players.end(); i++)
                if((*i)->id == curr_player->id) {
                    online_players.erase(i);
                    break;
                }

            sf::Packet quit_packet;
            quit_packet << PacketType::PLAYER_QUIT << curr_player->id;
            sendToEveryone(quit_packet);
            
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            unsigned char pos;
            event.packet >> pos;
            curr_player->player_inventory.swapWithMouseItem(&curr_player->player_inventory.inventory_arr[pos]);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            event.packet >> curr_player->player_inventory.selected_slot;
            break;
        }

        case PacketType::CHAT: {
            std::string message;
            event.packet >> message;
            std::string chat_format = (curr_player->name == "_" ? "Protagonist" : curr_player->name) + ": " + message;
            print::info(chat_format);
            
            sf::Packet chat_packet;
            chat_packet << PacketType::CHAT << chat_format;
            sendToEveryone(chat_packet);
            break;
        }

        default:;
    }
}

void players::onEvent(ServerBlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.type;
    sendToEveryone(packet);
}

void players::onEvent(ServerBlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS_CHANGE << event.block.getX() << event.block.getY() << event.break_stage;
    sendToEveryone(packet);
}

void players::onEvent(ServerLiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.liquid_type << event.liquid_level;
    sendToEveryone(packet);
}

void players::onEvent(ServerItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << event.x << event.y << event.id << (unsigned char)event.item_id;
    sendToEveryone(packet);
}

void players::onEvent(ServerItemDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_DELETION << (short)event.item_to_delete.getId();
    sendToEveryone(packet);
}

void players::onEvent(ServerItemMovementEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_MOVEMENT << event.moved_item.x <<  event.moved_item.y << event.moved_item.getId();
    sendToEveryone(packet);
}
