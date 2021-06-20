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
    
    // size of the packet are the first two bytes
    unsigned short size = buffer.size < 2 ? 0 : buffer.buffer[0] + (buffer.buffer[1] << 8);
    
    // packets can be merged so if multiple packets come in one piece,
    // it can process one buffer multiple times. Only refill it when its empty
    if(size + 3 > buffer.size) {
        // get packet/s and apply it to the buffer
        unsigned char temp_buffer[BUFFER_SIZE];
        int bytes_received = (int)recv(socket, (char*)temp_buffer, BUFFER_SIZE, 0);
        buffer.buffer = (unsigned char*)realloc(buffer.buffer, buffer.size + bytes_received);
        for(int i = 0; i < bytes_received; i++)
            buffer.buffer[buffer.size + i] = temp_buffer[i];
        buffer.size += bytes_received;
    }
    
    size = buffer.size < 2 ? 0 : buffer.buffer[0] + (buffer.buffer[1] << 8);
    
    // if bytes_received is 0 that means, that the other side disconnected
    if(buffer.size > 0) {
        // packet type is the third byte
        packet result((packets::packetType)buffer.buffer[2], size);
        for(unsigned short i = 0; i < size; i++)
            result.contents[i + 3] = buffer.buffer[i + 3];
        result.curr_pos = size + 3;
        // erase size + 3 for size variable and 1 for type
        
        //buffer.buffer.erase(buffer.buffer.begin(), buffer.buffer.begin() + size + 3);
        buffer.size -= size + 3;
        unsigned char* temp = new unsigned char[buffer.size];
        memcpy(temp, buffer.buffer + size + 3, buffer.size);
        buffer.buffer = (unsigned char*)realloc(buffer.buffer, buffer.size);
        memcpy(buffer.buffer, temp, buffer.size);
        
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
