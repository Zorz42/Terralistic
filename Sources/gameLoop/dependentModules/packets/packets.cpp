//
//  packets.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 15/01/2021.
//

#include <arpa/inet.h>
#include "packets.hpp"

#define BUFFER_SIZE 1024

packets::packet packets::getPacket(int socket) {
    std::vector<unsigned char> buffer(BUFFER_SIZE);
    long bytesReceived = recv(socket, &buffer[0], buffer.size(), 0);
    if(bytesReceived != -1)
        buffer.resize(bytesReceived);
    packet result((packets::packetType)buffer[0]);
    buffer.erase(buffer.begin());
    result.contents = buffer;
    return result;
}

void packets::sendPacket(int socket, packet packet_) {
    std::vector<unsigned char> content = {(unsigned char)packet_.type};
    for(char i : packet_.contents)
        content.push_back(i);
    send(socket, &content[0], content.size(), 0);
}

packets::packet& packets::packet::operator<<(char x) {
    contents.push_back(x);
    return *this;
}

packets::packet& packets::packet::operator<<(unsigned char x) {
    contents.push_back(x);
    return *this;
}

packets::packet& packets::packet::operator<<(short x) {
    for(int i = 1; i >= 0; i--)
        contents.push_back((x >> i * 8) & 0xFF);
    return *this;
}

packets::packet& packets::packet::operator<<(unsigned short x) {
    for(int i = 1; i >= 0; i--)
        contents.push_back((x >> i * 8) & 0xFF);
    return *this;
}

char packets::packet::getChar() {
    char result = contents.back();
    contents.pop_back();
    return result;
}

unsigned char packets::packet::getUChar() {
    unsigned char result = contents.back();
    contents.pop_back();
    return result;
}

short packets::packet::getShort() {
    short result = 0;
    for(int i = 0; i < 2; i++) {
        result += (int)contents.back() << i * 8;
        contents.pop_back();
    }
    return result;
}

unsigned short packets::packet::getUShort() {
    short result = 0;
    for(int i = 0; i < 2; i++) {
        result += (int)contents.back() << i * 8;
        contents.pop_back();
    }
    return result;
}
