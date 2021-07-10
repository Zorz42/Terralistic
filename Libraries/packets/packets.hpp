#ifndef packets_hpp
#define packets_hpp

#include <string>
#include <cstring>

enum class PacketType {DISCONNECT, CHUNK, BLOCK_CHANGE, PLAYER_JOIN, PLAYER_QUIT, PLAYER_MOVEMENT, ITEM_CREATION, ITEM_DELETION, ITEM_MOVEMENT, INVENTORY_CHANGE, INVENTORY_SWAP, HOTBAR_SELECTION, RIGHT_CLICK, STARTED_BREAKING, STOPPED_BREAKING, BLOCK_PROGRESS_CHANGE, SPAWN_POS, VIEW_SIZE_CHANGE, KICK, CHAT};

class Packet {
    unsigned char* contents = nullptr;
    unsigned short curr_pos;
    
    void allocateContents(unsigned short size, PacketType type);
    
    void copyBufferToContents(unsigned char *buffer, unsigned short size);
    
public:
    Packet(PacketType type, unsigned char* buffer, unsigned short size);
    Packet(PacketType type, unsigned short size);
    
    PacketType getType();
    
    template<class Type> Packet& operator<<(Type x);
    Packet& operator<<(std::string x);
    
    template<class Type> Type get();
    template<> std::string get<std::string>();
    
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

template<class Type>
Packet& Packet::operator<<(Type x) {
    memcpy(contents + curr_pos + 3, &x, sizeof(Type));
    curr_pos += sizeof(Type);
    return *this;
}

template<class Type>
Type Packet::get() {
    Type result = 0;
    memcpy(&result, contents + curr_pos + 3 - sizeof(Type), sizeof(Type));
    curr_pos -= sizeof(Type);
    return result;
}

#endif /* packets_hpp */
