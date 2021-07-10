#include "packets.hpp"

#ifdef _WIN32
#include <winsock2.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#endif

#define BUFFER_SIZE 2048

Packet::Packet(PacketType type, unsigned char* buffer, unsigned short size) : type(type) {
    contents = new unsigned char[size + 3];
    memcpy(contents + 3, buffer + 3, size * sizeof(unsigned char));
    curr_pos = size + 3;
}

Packet::Packet(PacketType type, unsigned short size) : type(type) {
    contents = new unsigned char[size + 3];
}

Packet& Packet::operator=(Packet& target) {
    target.contents = contents;
    contents = nullptr;
    target.curr_pos = curr_pos;
    target.type = type;
    return *this;
}

PacketType Packet::getType() {
    return type;
}

void Packet::send(int socket) const {
    contents[0] = (curr_pos - 3) & 255;
    contents[1] = ((curr_pos - 3) >> 8) & 255;
    contents[2] = (unsigned char)type;
    ::send(socket, (char*)contents, curr_pos, 0);
}

Packet::~Packet() {
    delete[] contents;
}

Packet PacketManager::getPacket() {
    /*
     Through TCP packets are array of bytes. In those packets you can
     serialize just about anything that has a fixed size. Int can be
     for example deconstructed into 4 bytes and then reconstructed at
     the other side.
     */
    
    // size of the packet are the first two bytes
    unsigned short size = buffer_size < 2 ? 0 : buffer[0] + (buffer[1] << 8);
    
    // packets can be merged so if multiple packets come in one piece,
    // it can process one buffer multiple times. Only refill it when its empty
    if(size + 3 > buffer_size) {
        // get packet/s and apply it to the buffer
        unsigned char temp_buffer[BUFFER_SIZE];
        int bytes_received = (int)recv(socket, (char*)temp_buffer, BUFFER_SIZE, 0);
        buffer = (unsigned char*)realloc(buffer, buffer_size + bytes_received);
        for(int i = 0; i < bytes_received; i++)
            buffer[buffer_size + i] = temp_buffer[i];
        buffer_size += bytes_received;
    }
    
    size = buffer_size < 2 ? 0 : buffer[0] + (buffer[1] << 8);
    
    // if bytes_received is 0 that means, that the other side disconnected
    if(buffer_size > 0) {
        // packet type is the third byte
        Packet result((PacketType)buffer[2], buffer, size);
        
        // erase size + 3 for size variable and 1 for type
        buffer_size -= size + 3;
        unsigned char* temp = new unsigned char[buffer_size];
        memcpy(temp, buffer + size + 3, buffer_size);
        buffer = (unsigned char*)realloc(buffer, buffer_size);
        memcpy(buffer, temp, buffer_size);
        
        return result;
    } else
        return Packet(PacketType::DISCONNECT, 0);
}

void PacketManager::sendPacket(const Packet& packet) const {
    packet.send(socket);
}
