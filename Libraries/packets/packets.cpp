#include "packets.hpp"

#ifdef _WIN32
#include <winsock2.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#endif

#define BUFFER_SIZE 2048

void Packet::allocateContents(unsigned short size, PacketType type) {
    contents = new unsigned char[size + 3];
    contents[0] = (size) & 255;
    contents[1] = ((size) >> 8) & 255;
    contents[2] = (unsigned char)type;
}

void Packet::copyBufferToContents(unsigned char *buffer, unsigned short size) {
    memcpy(contents + 3, buffer + 3, size * sizeof(unsigned char));
}

Packet::Packet(PacketType type, unsigned char* buffer, unsigned short size) {
    allocateContents(size, type);
    copyBufferToContents(buffer, size);
    curr_pos = size;
}

Packet::Packet(PacketType type, unsigned short size) {
    allocateContents(size, type);
    curr_pos = 0;
}

Packet& Packet::operator=(Packet& target) {
    target.contents = contents;
    contents = nullptr;
    target.curr_pos = curr_pos;
    return *this;
}

PacketType Packet::getType() {
    return (PacketType)contents[2];
}

void Packet::send(int socket) const {
    ::send(socket, (char*)contents, curr_pos + 3, 0);
}

template<>
std::string Packet::get<std::string>() {
    std::string result;
    curr_pos--;
    unsigned char size = contents[curr_pos + 3];
    curr_pos -= size;
    result.append(contents + curr_pos + 3, contents + curr_pos + 3 + size);
    return result;
}

Packet& Packet::operator<<(std::string x) {
    memcpy(contents + curr_pos + 3, &x[0], x.size());
    curr_pos += x.size();
    contents[curr_pos + 3] = x.size();
    curr_pos++;
    return *this;
}


Packet::~Packet() {
    delete[] contents;
}

unsigned short PacketManager::getPacketSizeFromBuffer() {
    return buffer_size == 0 ? 0 : buffer[0] + (buffer[1] << 8);
}

bool PacketManager::isBufferSufficient(unsigned short size) {
    return size + 3 <= buffer_size;
}

void PacketManager::appendToBuffer(unsigned char *appended_buffer, int appended_buffer_size) {
    buffer = (unsigned char*)realloc(buffer, buffer_size + appended_buffer_size);
    for(int i = 0; i < appended_buffer_size; i++)
        buffer[buffer_size + i] = appended_buffer[i];
    buffer_size += appended_buffer_size;
}

Packet PacketManager::getPacket() {
    unsigned short size = getPacketSizeFromBuffer();
    
    if(!isBufferSufficient(size)) {
        unsigned char temp_buffer[BUFFER_SIZE];
        int bytes_received = (int)recv(socket, (char*)temp_buffer, BUFFER_SIZE, 0);
        appendToBuffer(temp_buffer, bytes_received);
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
