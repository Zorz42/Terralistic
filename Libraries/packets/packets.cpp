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

#define BUFFER_SIZE 2048

packets::packet packets::getPacket(int socket, packetBuffer& buffer) {
    /*
     Through TCP packets are array of bytes. In those packets you can
     serialize just about anything that has a fixed size. Int can be
     for example deconstructed into 4 bytes and then reconstructed at
     the other side. Also data is stored in vector of unsigned char.
     */
    unsigned short size;
    
    // size of the packet are the first two bytes
    size = buffer.buffer.size() < 2 ? 0 : buffer.buffer[0] + (buffer.buffer[1] << 8);
    
    // packets can be merged so if multiple packets come in one piece,
    // it can process one buffer multiple times. Only refill it when its empty
    if(size + 3 > buffer.buffer.size()) {
        // get packet/s and apply it to the buffer
        std::vector<unsigned char> temp_buffer = std::vector<unsigned char>(BUFFER_SIZE);
        buffer.bytes_received = recv(socket, (char*)&temp_buffer[0], BUFFER_SIZE, 0);
        if(buffer.bytes_received != -1)
            temp_buffer.resize((unsigned int)(buffer.bytes_received));
        for(unsigned char i : temp_buffer)
            buffer.buffer.push_back(i);
    }
    
    size = buffer.buffer.size() < 2 ? 0 : buffer.buffer[0] + (buffer.buffer[1] << 8);
    
    // if bytes_received is 0 that means, that the other side disconnected
    if(buffer.bytes_received > 0) {
        // packet type is the third byte
        packet result((packets::packetType)buffer.buffer[2], size);
        for(unsigned short i = 0; i < size; i++)
            result.contents[i + 3] = buffer.buffer[i + 3];
        result.curr_pos = size + 3;
        // erase size + 3 for size variable and 1 for type
        buffer.buffer.erase(buffer.buffer.begin(), buffer.buffer.begin() + size + 3);
        return result;
    } else
        return packet(packets::DISCONNECT, 0);
}

void packets::sendPacket(int socket, const packet& packet_) {
    // first pack the size and type
    packet_.contents[0] = (packet_.curr_pos - 3) & 255;
    packet_.contents[1] = ((packet_.curr_pos - 3) >> 8) & 255;
    packet_.contents[2] = packet_.type;
    send(socket, (char*)packet_.contents, packet_.curr_pos, 0);
}
