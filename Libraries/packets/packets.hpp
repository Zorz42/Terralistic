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
#include <cstring>

namespace packets {

enum packetType {DISCONNECT, PING, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT};

struct packetBuffer {
    std::vector<unsigned char> buffer;
    long bytes_received;
};

struct packet {
    packet(packetType type, unsigned short size) : type(type) {
        contents = new unsigned char[size + 3];
    }
    unsigned char* contents = nullptr;
    packetType type;
    unsigned short curr_pos = 3;
    
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
    
    void operator=(packet& target) {
        target.contents = contents;
        contents = nullptr;
        target.curr_pos = curr_pos;
        target.type = type;
    }
    
    ~packet() {
        if(contents)
            delete[] contents;
    }
};

packet getPacket(int socket, packetBuffer& buffer);
void sendPacket(int socket, const packet& content);

}

#endif /* packets_hpp */
