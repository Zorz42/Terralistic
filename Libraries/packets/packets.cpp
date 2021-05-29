//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#include "packets.hpp"

#ifdef _WIN32
#include <winsock2.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#endif

#define BUFFER_SIZE 1024

#define define_operator(T) \
packets::packet& packets::packet::operator<<(T x) { \
    for(int i = sizeof(T) - 1; i >= 0; i--) \
        contents.push_back((x >> i * 8) & 0xFF); \
    return *this; \
}

#define define_get(T, name) \
T packets::packet::name() { \
    T result = 0; \
    for(int i = 0; i < sizeof(T); i++) { \
        result += (T)contents.back() << i * 8; \
        contents.pop_back(); \
    } \
    return result; \
}

packets::packet packets::getPacket(int socket, std::vector<unsigned char>& buffer, long& bytes_received) {
    /*
     Through TCP packets are array of bytes. In those packets you can
     serialize just about anything that has a fixed size. Int can be
     for example deconstructed into 4 bytes and then reconstructed at
     the other side. Also data is stored in vector of unsigned char.
     */
    unsigned short size;
    
    // size of the packet are the first two bytes
    size = buffer.size() < 2 ? 0 : buffer[0] + (buffer[1] << 8);
    
    // packets can be merged so if multiple packets come in one piece,
    // it can process one buffer multiple times. Only refill it when its empty
    if(size + 3 > buffer.size()) {
        // get packet/s and apply it to the buffer
        std::vector<unsigned char> temp_buffer = std::vector<unsigned char>(BUFFER_SIZE);
        bytes_received = recv(socket, (char*)&temp_buffer[0], BUFFER_SIZE, 0);
        if(bytes_received != -1)
            temp_buffer.resize((unsigned int)(bytes_received));
        for(unsigned char i : temp_buffer)
            buffer.push_back(i);
    }
    
    size = buffer.size() < 2 ? 0 : buffer[0] + (buffer[1] << 8);
    
    // if bytes_received is 0 that means, that the other side disconnected
    if(bytes_received > 0) {
        // packet type is the third byte
        packet result((packets::packetType)buffer[2]);
        for(unsigned short i = 0; i < size; i++)
            result.contents.push_back(buffer[i + 3]);
        // erase size + 2 for size variable and 1 for type
        buffer.erase(buffer.begin(), buffer.begin() + size + 3);
        return result;
    } else
        return packet(packets::DISCONNECT);
}

void packets::sendPacket(int socket, const packet& packet_) {
    // first pack the size and type
    std::vector<unsigned char> content = {(unsigned char)(packet_.contents.size() & 255), (unsigned char)((packet_.contents.size() >> 8) & 255), (unsigned char)packet_.type};
    // add contents and send
    for(char i : packet_.contents)
        content.push_back((unsigned char &&)i);
    send(socket, (char*)&content[0], content.size(), 0);
}

// define << for all needed types

define_operator(char)
define_operator(unsigned char)
define_operator(short)
define_operator(unsigned short)
define_operator(int)
define_operator(unsigned int)

// define get[type]() for all needed types

define_get(char, getChar)
define_get(unsigned char, getUChar)
define_get(short, getShort)
define_get(unsigned short, getUShort)
define_get(int, getInt)
define_get(unsigned int, getUInt)


