//
//  packets.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#ifndef packets_hpp
#define packets_hpp

#include <vector>
#include <string>

namespace packets {

enum packetType {DISCONNECT, PING, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT};

struct packet {
    packet(packetType type, unsigned short size) : type(type) {
        if(size)
            contents = new unsigned char[size];
    }
    unsigned char* contents = nullptr;
    packetType type;
    unsigned short curr_pos = 0;
    
    template<class T>
    packet& operator<<(T x) {
        memcpy(contents + curr_pos, &x, sizeof(T));
        curr_pos += sizeof(T);
        return *this;
    }
    
    packet& operator<<(std::string x) {
        memcpy(contents + curr_pos, &x[0], x.size());
        curr_pos += x.size();
        contents[curr_pos++] = x.size();
        return *this;
    }
    
    template<class T>
    T get() {
        T result = 0;
        memcpy(&result, contents + curr_pos - sizeof(T), sizeof(T));
        curr_pos -= sizeof(T);
        return result;
    }
    
    template<>
    std::string get<std::string>() {
        std::string result;
        unsigned char size = contents[--curr_pos];
        result.reserve(size);
        for(int i = 0; i < size; i++)
            result.push_back(contents[curr_pos - size + i]);
        curr_pos -= size;
        return result;
    }
    
    ~packet() {
        //if(contents) {
            //delete[] contents;
            //contents = nullptr;
        //}
    }
};

packet getPacket(int socket, std::vector<unsigned char>& buffer, long& bytes_received);
void sendPacket(int socket, const packet& content);

}

#endif /* packets_hpp */
