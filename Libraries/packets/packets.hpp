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
    
    unsigned short getPacketSizeFromBuffer();
    bool isBufferSufficient(unsigned short size);
    void appendToBuffer(unsigned char *appended_buffer, int appended_buffer_size);
    bool otherSideDisconneceted();
    PacketType getTypeFromBuffer();
    void eraseFrontOfBuffer(unsigned short size);
    int socket;
    bool socket_is_set = false;
    
public:
    int getSocket() const;
    void bindToSocket(int sock);
    bool isSocketSet() const;
    
    Packet getPacket();
    void sendPacket(const Packet& packet) const;
};

#endif /* packets_hpp */
