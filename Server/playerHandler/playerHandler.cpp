//
//  playerHandler.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 30/01/2021.
//

#include "core.hpp"

#ifdef WIN32
#include <winsock2.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#endif

#include "playerHandler.hpp"
#include "blockEngine.hpp"
#include "print.hpp"

playerHandler::player* playerHandler::getPlayerByConnection(networking::connection* conn) {
    for(playerHandler::player& player : playerHandler::players)
        if(player.conn == conn)
            return &player;
    return nullptr;
}

PACKET_LISTENER(packets::PLAYER_MOVEMENT)
    playerHandler::player* player = playerHandler::getPlayerByConnection(&connection);
    player->flipped = packet.getChar();
    player->y = packet.getInt();
    player->x = packet.getInt();

    packets::packet movement_packet(packets::PLAYER_MOVEMENT);
    movement_packet << player->x << player->y << (char)player->flipped << player->id;
    networking::sendToEveryone(movement_packet, player->conn);
PACKET_LISTENER_END

PACKET_LISTENER(packets::PLAYER_JOIN)
    static unsigned int curr_id = 0;
    playerHandler::player player(curr_id++);
    player.conn = &connection;
    player.y = blockEngine::getSpawnY() - BLOCK_WIDTH * 2;
    player.x = blockEngine::getSpawnX();

    packets::packet spawn_packet(packets::SPAWN_POS);
    spawn_packet << player.y << player.x;
    connection.sendPacket(spawn_packet);
    
    for(playerHandler::player& i : playerHandler::players) {
        packets::packet join_packet(packets::PLAYER_JOIN);
        join_packet << i.x << i.y << i.id;
        player.conn->sendPacket(join_packet);
    }
    for(itemEngine::item& i : itemEngine::items) {
        packets::packet item_packet(packets::ITEM_CREATION);
        item_packet << i.x << i.y << i.getId() << (char)i.getItemId();
        player.conn->sendPacket(item_packet);
    }
    
    packets::packet join_packet_out(packets::PLAYER_JOIN);
    join_packet_out << player.x << player.y << player.id;
    networking::sendToEveryone(join_packet_out, player.conn);
    
    playerHandler::players.push_back(player);
    
    print::info(player.conn->ip + " connected (" + std::to_string(playerHandler::players.size()) + " players online)");
PACKET_LISTENER_END

PACKET_LISTENER(packets::DISCONNECT)
    print::info(connection.ip + " disconnected (" + std::to_string(playerHandler::players.size() - 1) + " players online)");
    playerHandler::player* player = playerHandler::getPlayerByConnection(&connection);
#ifndef WIN32
    close(connection.socket);
#endif
    for(networking::connection& conn : networking::connections)
        if(conn.socket == connection.socket) {
            conn.socket = -1;
            conn.ip.clear();
            break;
        }
    
    packets::packet quit_packet(packets::PLAYER_QUIT);
    quit_packet << player->id;
    
    for(auto i = playerHandler::players.begin(); i != playerHandler::players.end(); i++)
        if(i->id == player->id) {
            playerHandler::players.erase(i);
            break;
        }
    networking::sendToEveryone(quit_packet);
PACKET_LISTENER_END

PACKET_LISTENER(packets::INVENTORY_SWAP)
    unsigned char pos = packet.getUChar();
    playerHandler::player* player = playerHandler::getPlayerByConnection(&connection);
    player->inventory.swapWithMouseItem(&player->inventory.inventory[pos]);
PACKET_LISTENER_END

PACKET_LISTENER(packets::HOTBAR_SELECTION)
    playerHandler::getPlayerByConnection(&connection)->inventory.selected_slot = packet.getChar();
PACKET_LISTENER_END

void playerHandler::lookForItems() {
    for(unsigned long i = 0; i < itemEngine::items.size(); i++) {
        for(playerHandler::player& player : playerHandler::players)
            if(abs(itemEngine::items[i].x / 100 + BLOCK_WIDTH / 2  - player.x - 14) < 50 && abs(itemEngine::items[i].y / 100 + BLOCK_WIDTH / 2 - player.y - 25) < 50) {
                char result = player.inventory.addItem(itemEngine::items[i].getItemId(), 1);
                if(result != -1) {
                    packets::packet item_receive_packet(packets::INVENTORY_CHANGE);
                    item_receive_packet << (unsigned char)player.inventory.inventory[result].item_id << (unsigned short)player.inventory.inventory[result].getStack() << result;
                    player.conn->sendPacket(item_receive_packet);
                    itemEngine::items[i].destroy();
                    itemEngine::items.erase(itemEngine::items.begin() + i);
                }
            }
    }
}
