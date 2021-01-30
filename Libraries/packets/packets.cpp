//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#include <arpa/inet.h>
#include "packets.hpp"
#include <iostream>

#define BUFFER_SIZE 1024
#define define_operator(T) packets::packet& packets::packet::operator<<(T x) {for(int i = sizeof(T) - 1; i >= 0; i--) contents.push_back((x >> i * 8) & 0xFF); return *this;}
#define define_get(T, name) T packets::packet::name() {T result = 0; for(int i = 0; i < sizeof(T); i++) {result += (T)contents.back() << i * 8; contents.pop_back();} return result;}

packets::packet packets::getPacket(int socket) {
    static std::vector<unsigned char> buffer;
    static long bytesReceived;
    if(buffer.empty()) {
        buffer = std::vector<unsigned char>(BUFFER_SIZE);
        bytesReceived = recv(socket, &buffer[0], buffer.size(), 0);
        if(bytesReceived != -1)
            buffer.resize(bytesReceived);
    }
    
    if(bytesReceived) {
        unsigned short size = buffer[0] + (buffer[1] << 8);
        packet result((packets::packetType)buffer[2]);
        for(unsigned short i = 0; i < size; i++)
            result.contents.push_back(buffer[i + 3]);
        buffer.erase(buffer.begin(), buffer.begin() + size + 3);
        return result;
    } else
        return packet(packets::DISCONNECT);
}

void packets::sendPacket(int socket, packet packet_) {
    std::vector<unsigned char> content = {(unsigned char)(packet_.contents.size() & 255), (unsigned char)((packet_.contents.size() >> 8) & 255), (unsigned char)packet_.type};
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
