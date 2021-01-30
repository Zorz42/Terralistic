//
//  networkingModule.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include <iostream>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <thread>
#include "networkingModule.hpp"
#include "blockEngine.hpp"
#include "gameLoop.hpp"
#include "otherPlayers.hpp"
#include "itemEngine.hpp"
#include "objectedGraphicsLibrary.hpp"
#include "singleWindowLibrary.hpp"
#include "playerHandler.hpp"

#define BUFFER_SIZE 1024
#define PORT 33770

int sock;
ogl::texture connecting_text;

packets::packet networking::getPacket() {
    return packets::getPacket(sock);
}

void networking::sendPacket(packets::packet packet_) {
    packets::sendPacket(sock, packet_);
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
    struct sockaddr_in serv_addr;
    if((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        return false;
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);
    return inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) > 0 && connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) >= 0;
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
                blockEngine::blockType type = (blockEngine::blockType)packet.getUChar();
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
                for(auto i = players::players.begin(); i != players::players.end(); i++)
                    if(i->id == id) {
                        i->flipped = packet.getChar();
                        i->y = packet.getInt();
                        i->x = packet.getInt();
                        break;
                    }
                break;
            }
            case packets::ITEM_CREATION: {
                itemEngine::itemType type = (itemEngine::itemType)packet.getChar();
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
                for(auto i = itemEngine::items.begin(); i != itemEngine::items.end(); i++)
                    if(i->id == id) {
                        i->x = x;
                        i->y = y;
                        break;
                    }
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
