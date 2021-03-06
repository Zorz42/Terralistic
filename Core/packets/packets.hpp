//
//  packets.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#ifndef packets_hpp
#define packets_hpp

namespace packets {

enum packetType {DISCONNECT, PING, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_BREAK_PROGRESS_CHANGE, SPAWN_POS};

struct packet {
    packet(packetType type) : type(type) {}
    std::vector<unsigned char> contents;
    packetType type;
    packet& operator<<(char x);
    packet& operator<<(unsigned char x);
    packet& operator<<(short x);
    packet& operator<<(unsigned short x);
    packet& operator<<(int x);
    packet& operator<<(unsigned int x);
    char getChar();
    unsigned char getUChar();
    short getShort();
    unsigned short getUShort();
    int getInt();
    unsigned int getUInt();
};

packet getPacket(int socket);
void sendPacket(int socket, const packet& content);

}

#endif /* packets_hpp */
