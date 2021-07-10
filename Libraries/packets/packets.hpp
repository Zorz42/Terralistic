#ifndef packets_hpp
#define packets_hpp

#include <string>
#include <cstring>

enum class PacketType {DISCONNECT, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT};

class Packet {
    unsigned char* contents = nullptr;
    PacketType type;
    unsigned short curr_pos = 3;
public:
    
    Packet(PacketType type, unsigned char* buffer, unsigned short size);
    Packet(PacketType type, unsigned short size);
    
    PacketType getType();
    
    template<class T>
    Packet& operator<<(T x) {
        memcpy(contents + curr_pos, &x, sizeof(T));
        curr_pos += sizeof(T);
        return *this;
    }
    
    Packet& operator<<(std::string x) {
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
    
    Packet& operator=(Packet& target);
    void send(int socket) const;
    ~Packet();
};

class PacketManager {
    unsigned char* buffer = nullptr;
    int buffer_size = 0;
public:
    int socket = -1;
    
    Packet getPacket();
    void sendPacket(const Packet& packet) const;
};

#endif /* packets_hpp */
