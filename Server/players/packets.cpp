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

void players::onPacket(Packet& packet, connection& conn) {
    player* curr_player = getPlayerByConnection(&conn);
    switch (packet.getType()) {
        case PacketType::STARTED_BREAKING: {
            auto y = packet.get<unsigned short>(), x = packet.get<unsigned short>();
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
            auto y = packet.get<unsigned short>(), x = packet.get<unsigned short>();
            rightClickEvent(parent_blocks->getBlock(x, y), curr_player);
            break;
        }

        case PacketType::CHUNK: {
            auto x = packet.get<unsigned short>(), y = packet.get<unsigned short>();
            Packet chunk_packet(PacketType::CHUNK, (sizeof(unsigned char) + sizeof(unsigned char) + sizeof(unsigned char) + sizeof(unsigned char)) * 16 * 16 + sizeof(x) + sizeof(y));
            for(int i = 0; i < 16 * 16; i++) {
                block block = parent_blocks->getBlock((x << 4) + 15 - i % 16, (y << 4) + 15 - i / 16);
                chunk_packet << (unsigned char)block.getType() << (unsigned char)block.getLiquidType() << (unsigned char)block.getLiquidLevel() << (unsigned char)block.getLightLevel();
            }
            chunk_packet << y << x;
            conn.sendPacket(chunk_packet);
            break;
        }

        case PacketType::VIEW_SIZE_CHANGE: {
            auto width = packet.get<unsigned short>(), height = packet.get<unsigned short>();
            curr_player->sight_width = width;
            curr_player->sight_height = height;
            break;
        }

        case PacketType::PLAYER_MOVEMENT: {
            curr_player->flipped = packet.get<char>();
            curr_player->y = packet.get<int>();
            curr_player->x = packet.get<int>();

            Packet movement_packet(PacketType::PLAYER_MOVEMENT, sizeof(curr_player->x) + sizeof(curr_player->y) + sizeof(char) + sizeof(curr_player->id));
            movement_packet << curr_player->x << curr_player->y << (char)curr_player->flipped << curr_player->id;
            manager->sendToEveryone(movement_packet, curr_player->conn);
            break;
        }

        case PacketType::PLAYER_JOIN: {
            std::string name = packet.get<std::string>();

            curr_player = getPlayerByName(name);
            curr_player->conn = &conn;

            Packet spawn_packet(PacketType::SPAWN_POS, sizeof(curr_player->y) + sizeof(curr_player->x));
            spawn_packet << curr_player->y << curr_player->x;
            conn.sendPacket(spawn_packet);

            for(player* player : online_players) {
                Packet join_packet(PacketType::PLAYER_JOIN, sizeof(player->x) + sizeof(player->y) + sizeof(player->id) + (int)player->name.size() + 1);
                join_packet << player->x << player->y << player->id << player->name;
                curr_player->conn->sendPacket(join_packet);
            }

            for(const item& i : parent_items->getItems()) {
                Packet item_packet(PacketType::ITEM_CREATION, sizeof(i.x) + sizeof(i.y) + sizeof(i.getId()) + sizeof(char));
                item_packet << i.x << i.y << i.getId() << (char)i.getItemId();
                curr_player->conn->sendPacket(item_packet);
            }

            for(inventoryItem& i : curr_player->player_inventory.inventory_arr) // send the whole inventory
                if(i.getId() != ItemType::NOTHING)
                    i.sendPacket();

            Packet join_packet_out(PacketType::PLAYER_JOIN, sizeof(curr_player->x) + sizeof(curr_player->y) + sizeof(curr_player->id) + (int)curr_player->name.size() + 1);
            join_packet_out << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
            manager->sendToEveryone(join_packet_out, curr_player->conn);

            curr_player->conn->registered = true;

            online_players.push_back(curr_player);

            print::info(curr_player->name + " (" + curr_player->conn->ip + ") connected (" + std::to_string(online_players.size()) + " players online)");
            break;
        }

        case PacketType::DISCONNECT: {
            print::info(curr_player->name + " (" + curr_player->conn->ip + ") disconnected (" + std::to_string(online_players.size() - 1) + " players online)");
            player* player = getPlayerByConnection(&conn);
#ifdef _WIN32
            closesocket(conn.getSocket());
#else
            close(conn.getSocket());
#endif
            for(connection& i : manager->connections)
                if(i.getSocket() == conn.getSocket()) {
                    i.setSocket(-1);
                    i.ip.clear();
                    break;
                }

            Packet quit_packet(PacketType::PLAYER_QUIT, sizeof(player->id));
            quit_packet << player->id;

            for(auto i = online_players.begin(); i != online_players.end(); i++)
                if((*i)->id == player->id) {
                    online_players.erase(i);
                    break;
                }
            manager->sendToEveryone(quit_packet);
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            auto pos = packet.get<unsigned char>();
            player* player = getPlayerByConnection(&conn);
            player->player_inventory.swapWithMouseItem(&player->player_inventory.inventory_arr[pos]);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            curr_player->player_inventory.selected_slot = packet.get<char>();
            break;
        }

        case PacketType::CHAT: {
            std::string chat_format = (curr_player->name == "_" ? "Protagonist" : curr_player->name) + ": " + packet.get<std::string>();
            print::info(chat_format);
            Packet chat_packet(PacketType::CHAT, (int)chat_format.size() + 1);
            chat_packet << chat_format;
            manager->sendToEveryone(chat_packet);
            break;
        }

        default:;
    }
}
