//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#include <arpa/inet.h>
#include "packets.hpp"

#define BUFFER_SIZE 1024
#define define_operator(T) packets::packet& packets::packet::operator<<(T x) {for(int i = sizeof(T) - 1; i >= 0; i--) contents.push_back((x >> i * 8) & 0xFF); return *this;}
#define define_get(T, name) T packets::packet::name() {T result = 0; for(int i = 0; i < sizeof(T); i++) {result += (T)contents.back() << i * 8; contents.pop_back();} return result;}

packets::packet packets::getPacket(int socket) {
    std::vector<unsigned char> buffer(BUFFER_SIZE);
    long bytesReceived = recv(socket, &buffer[0], buffer.size(), 0);
    if(bytesReceived != -1)
        buffer.resize(bytesReceived);
    
    if(bytesReceived) {
        packet result((packets::packetType)buffer[0]);
        buffer.erase(buffer.begin());
        result.contents = buffer;
        return result;
    }
    else {
        packet result(packets::DISCONNECT);
        return result;
    }
}

void packets::sendPacket(int socket, packet packet_) {
    std::vector<unsigned char> content = {(unsigned char)packet_.type};
    for(char i : packet_.contents)
        content.push_back(i);
    send(socket, &content[0], content.size(), 0);
}

define_operator(char)
define_operator(unsigned char)
define_operator(short)
define_operator(unsigned short)
define_operator(int)
define_operator(unsigned int)

define_get(char, getChar)
define_get(unsigned char, getUChar)
define_get(short, getShort)
define_get(unsigned short, getUShort)
define_get(int, getInt)
define_get(unsigned int, getUInt)
