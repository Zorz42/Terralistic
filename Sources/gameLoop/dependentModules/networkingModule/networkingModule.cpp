//
//  networkingModule.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include <iostream>
#ifdef WIN32
#include <windows.h>
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#endif
#include <thread>
#include "networkingModule.hpp"
#include "blockEngine.hpp"
#include "gameLoop.hpp"
#include "otherPlayers.hpp"
#include "itemEngine.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"
#include "inventory.hpp"

#define BUFFER_SIZE 1024
#define PORT 33770
#define PORT_STR "33770"

int sock;
ogl::texture connecting_text;

packets::packet networking::getPacket() {
    return packets::getPacket(sock);
}

void networking::sendPacket(packets::packet packet_) {
    packets::sendPacket(sock, std::move(packet_));
}

void networking::init() {
    connecting_text.loadFromText("Connecting to server", {255, 255, 255});
    connecting_text.scale = 3;
}

bool networking::establishConnection(const std::string &ip) {
    swl::setDrawColor(0, 0, 0);
        swl::clear();
        connecting_text.render();
        swl::update();

    #ifdef WIN32
        WSADATA wsaData;
        if(WSAStartup(MAKEWORD(2,2), &wsaData) != 0)
            return false;
    #endif

        sockaddr_in serv_addr{};
        if((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
            return false;
        serv_addr.sin_family = AF_INET;
        serv_addr.sin_port = htons(PORT);

    #ifdef WIN32
        struct addrinfo *result = nullptr, hints{};
        if(getaddrinfo(ip.c_str(), (const char *)PORT_STR, &hints, &result) != 0)
            return false;

        return connect(sock, result->ai_addr, (int)result->ai_addrlen) != SOCKET_ERROR;

    #else
        if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0)
            return false;

        return connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) >= 0;
    #endif
}

void networking::downloadWorld() {
    packets::packet join_packet(packets::PLAYER_JOIN);
    join_packet << playerHandler::position_x << playerHandler::position_y;
    sendPacket(join_packet);
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++) {
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++) {
            packets::packet chunk_packet = getPacket();
            blockEngine::chunk& chunk = blockEngine::getChunk(x, y);
            for(int i = 0; i < 16 * 16; i++)
                chunk.blocks[i % 16][i / 16].block_id = (blockEngine::blockType)chunk_packet.getChar();
            sendPacket({packets::PING});
        }
    }
    for(int y = 0; y < blockEngine::world_height; y++)
        for(int x = 0; x < blockEngine::world_width; x++)
            blockEngine::getBlock(x, y).update(x, y);
}

void listenerLoop() {
    while(gameLoop::running) {
        packets::packet packet = networking::getPacket();
        switch(packet.type) {
            case packets::BLOCK_CHANGE: {
                auto type = (blockEngine::blockType)packet.getUChar();
                unsigned short y = packet.getUShort(), x = packet.getUShort();
                blockEngine::removeNaturalLight(x);
                blockEngine::getBlock(x, y).setBlockType(type, x, y, false);
                blockEngine::setNaturalLight(x);
                blockEngine::getBlock(x, y).update(x, y);
                blockEngine::getBlock(x, y).light_update(x, y);
                blockEngine::updateNearestBlocks(x, y);
                break;
            }
            case packets::PLAYER_JOIN: {
                players::player player;
                player.id = packet.getUShort();
                player.y = packet.getInt();
                player.x = packet.getInt();
                players::players.push_back(player);
                break;
            }
            case packets::PLAYER_QUIT: {
                unsigned short id = packet.getUShort();
                for(auto i = players::players.begin(); i != players::players.end(); i++)
                    if(i->id == id) {
                        players::players.erase(i);
                        break;
                    }
                break;
            }
            case packets::PLAYER_MOVEMENT: {
                unsigned short id = packet.getUShort();
                for(auto & player : players::players)
                    if(player.id == id) {
                        player.flipped = packet.getChar();
                        player.y = packet.getInt();
                        player.x = packet.getInt();
                        break;
                    }
                break;
            }
            case packets::ITEM_CREATION: {
                auto type = (itemEngine::itemType)packet.getChar();
                unsigned short id = packet.getUShort();
                int y = packet.getInt(), x = packet.getInt();
                itemEngine::spawnItem(type, x, y);
                itemEngine::items.back().id = id;
                break;
            }
            case packets::ITEM_DELETION: {
                unsigned short id = packet.getUShort();
                for(auto i = itemEngine::items.begin(); i != itemEngine::items.end(); i++)
                    if(i->id == id) {
                        itemEngine::items.erase(i);
                        break;
                    }
                break;
            }
            case packets::ITEM_MOVEMENT: {
                unsigned short id = packet.getUShort();
                int y = packet.getInt(), x = packet.getInt();
                for(auto & item : itemEngine::items)
                    if(item.id == id) {
                        item.x = x;
                        item.y = y;
                        break;
                    }
                break;
            }
            case packets::INVENTORY_ITEM_RECEIVED: {
                inventory::addItemToInventory((itemEngine::itemType)packet.getUChar(), 1);
                break;
            }
            default:;
        }
    }
}

void networking::spawnListener() {
    std::thread listener(listenerLoop);
    listener.detach();
}
