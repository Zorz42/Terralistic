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

#define BUFFER_SIZE 1024
#define PORT 33770

int sock;

packets::packet networking::getPacket() {
    return packets::getPacket(sock);
}

void networking::sendPacket(packets::packet packet_) {
    packets::sendPacket(sock, packet_);
}

bool networking::establishConnection(const std::string &ip) {
    struct sockaddr_in serv_addr;
    if((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        return false;

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(PORT);
       
    // Convert IPv4 and IPv6 addresses from text to binary form
    if(inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr)<=0)
        return false;
    
    return connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) >= 0;
}

void networking::downloadWorld() {
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
                blockEngine::getBlock(x, y).setBlockType(type, x, y, false);
                blockEngine::updateNearestBlocks(x, y);
                break;
            }
            case packets::DISCONNECT:
            case packets::PING:
            case packets::CHUNK:;
        }
    }
}

void networking::spawnListener() {
    std::thread listener(listenerLoop);
    listener.detach();
}
