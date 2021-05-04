//
//  playerHandler.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 30/01/2021.
//

#ifdef WIN32
#include <winsock2.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "playerHandler.hpp"
#include "print.hpp"
#include "map.hpp"

player* getPlayerByConnection(connection* conn) {
    for(player& player : players)
        if(player.conn == conn)
            return &player;
    return nullptr;
}

void lookForItems(map& world_map) {
    for(unsigned long i = 0; i < world_map.items.size(); i++) {
        for(player& player : players)
            if(abs(world_map.items[i].x / 100 + BLOCK_WIDTH / 2  - player.x - 14) < 50 && abs(world_map.items[i].y / 100 + BLOCK_WIDTH / 2 - player.y - 25) < 50) {
                char result = player.inventory.addItem(world_map.items[i].getItemId(), 1);
                if(result != -1) {
                    packets::packet item_receive_packet(packets::INVENTORY_CHANGE);
                    item_receive_packet << (unsigned char)player.inventory.inventory[result].item_id << (unsigned short)player.inventory.inventory[result].getStack() << result;
                    player.conn->sendPacket(item_receive_packet);
                    world_map.items[i].destroy(world_map);
                    world_map.items.erase(world_map.items.begin() + i);
                }
            }
    }
}

void playerHandler::onPacket(packets::packet& packet, connection& conn) {
    player* curr_player = getPlayerByConnection(&conn);
    switch (packet.type) {
        case packets::PLAYER_MOVEMENT: {
            curr_player->flipped = packet.getChar();
            curr_player->y = packet.getInt();
            curr_player->x = packet.getInt();

            packets::packet movement_packet(packets::PLAYER_MOVEMENT);
            movement_packet << curr_player->x << curr_player->y << (char)curr_player->flipped << curr_player->id;
            manager->sendToEveryone(movement_packet, curr_player->conn);
            break;
        }
            
        case packets::PLAYER_JOIN: {
            static unsigned int curr_id = 0;
            player curr_player(curr_id++);
            curr_player.conn = &conn;
            curr_player.y = playerHandler::world_map->getSpawnY() - BLOCK_WIDTH * 2;
            curr_player.x = playerHandler::world_map->getSpawnX();

            packets::packet spawn_packet(packets::SPAWN_POS);
            spawn_packet << curr_player.y << curr_player.x;
            conn.sendPacket(spawn_packet);
            
            for(player& i : players) {
                packets::packet join_packet(packets::PLAYER_JOIN);
                join_packet << i.x << i.y << i.id;
                curr_player.conn->sendPacket(join_packet);
            }
            for(map::item& i : playerHandler::world_map->items) {
                packets::packet item_packet(packets::ITEM_CREATION);
                item_packet << i.x << i.y << i.getId() << (char)i.getItemId();
                curr_player.conn->sendPacket(item_packet);
            }
            
            packets::packet join_packet_out(packets::PLAYER_JOIN);
            join_packet_out << curr_player.x << curr_player.y << curr_player.id;
            manager->sendToEveryone(join_packet_out, curr_player.conn);
            
            players.push_back(curr_player);
            
            print::info(curr_player.conn->ip + " connected (" + std::to_string(players.size()) + " players online)");
            break;
        }
            
        case packets::DISCONNECT: {
            print::info(conn.ip + " disconnected (" + std::to_string(players.size() - 1) + " players online)");
            player* player = getPlayerByConnection(&conn);
            #ifndef WIN32
                close(conn.socket);
            #endif
            for(connection& i : manager->connections)
                if(i.socket == conn.socket) {
                    i.socket = -1;
                    i.ip.clear();
                    break;
                }
            
            packets::packet quit_packet(packets::PLAYER_QUIT);
            quit_packet << player->id;
            
            for(auto i = players.begin(); i != players.end(); i++)
                if(i->id == player->id) {
                    players.erase(i);
                    break;
                }
            manager->sendToEveryone(quit_packet);
            break;
        }
            
        case packets::INVENTORY_SWAP: {
            unsigned char pos = packet.getUChar();
            player* player = getPlayerByConnection(&conn);
            player->inventory.swapWithMouseItem(&player->inventory.inventory[pos]);
            break;
        }
            
        case packets::HOTBAR_SELECTION: {
            curr_player->inventory.selected_slot = packet.getChar();
            break;
        }
            
        default:;
    }
}
