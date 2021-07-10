#include "packets.hpp"

#ifdef _WIN32
#include <winsock2.h>
#else
#include <sys/socket.h>
#include <arpa/inet.h>
#endif

#define BUFFER_SIZE 2048

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
        Packet result((PacketType)buffer[2], size);
        for(unsigned short i = 0; i < size; i++)
            result.contents[i + 3] = buffer[i + 3];
        result.curr_pos = size + 3;
        // erase size + 3 for size variable and 1 for type
        
        //buffer.erase(buffer.begin(), buffer.begin() + size + 3);
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
    // first pack the size and type
    packet.contents[0] = (packet.curr_pos - 3) & 255;
    packet.contents[1] = ((packet.curr_pos - 3) >> 8) & 255;
    packet.contents[2] = (unsigned char)packet.type;
    send(socket, (char*)packet.contents, packet.curr_pos, 0);
}
