#include "networking.hpp"
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include "exception.hpp"

void _socketSetBlocking(int socket_handle, bool blocking) {
#ifdef _WIN32
    unsigned long mode = blocking ? 0 : 1;
    if(ioctlsocket(socket_handle, FIONBIO, &mode) != 0)
        throw Exception("Could not set socket to blocking");
#else
    int flags = fcntl(socket_handle, F_GETFL, 0);
    if(flags == -1)
        throw Exception("Could not set socket to blocking");
    flags = blocking ? (flags & ~O_NONBLOCK) : (flags | O_NONBLOCK);
    if(fcntl(socket_handle, F_SETFL, flags) != 0)
        throw Exception("Could not set socket to blocking");
#endif
}

void TcpSocket::handleError() {
    if(errno == EAGAIN || errno == EINPROGRESS)
        return;

    switch(errno) {
        case EWOULDBLOCK:  return;
        case ECONNABORTED: disconnected = true; return;
        case ECONNRESET:   disconnected = true; return;
        case ETIMEDOUT:    disconnected = true; return;
        case ENETRESET:    disconnected = true; return;
        case ENOTCONN:     disconnected = true; return;
        case EPIPE:        disconnected = true; return;
        default:           throw Exception("Socket error");
    }
}

void TcpSocket::send(const void* obj, unsigned int size) {
    unsigned int sent = 0;
    while(sent < size) {
        int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)obj + sent), size - sent, 0);
        sent += curr_sent;
        
        if(curr_sent < 0) {
            handleError();
            break;
        }
    }
}

void TcpSocket::send(Packet& packet) {
    unsigned int size = (unsigned int)packet.getDataSize();
    unsigned int prev_buffer_size = (unsigned int)packet_buffer.size();
    packet_buffer.resize(packet_buffer.size() + sizeof(unsigned int) + size);
    std::memcpy(&packet_buffer[prev_buffer_size], &size, sizeof(unsigned int));
    std::memcpy(&packet_buffer[prev_buffer_size + sizeof(unsigned int)], packet.getData(), size);
    
    if(packet_buffer.size() > 65536)
        flushPacketBuffer();
}

bool TcpSocket::receive(void* obj, unsigned int size) {
    unsigned int received = 0;
    while(received < size) {
        int curr_received = (int)::read(socket_handle, (void*)((unsigned long)obj + received), size - received);
        received += curr_received;
        
        if(curr_received == 0) {
            disconnected = true;
            return false;
        } else if(curr_received < 0) {
            handleError();
            return false;
        }
    }
    
    return true;
}

bool TcpSocket::receive(Packet& packet) {
    unsigned int size;
    if(!receive(&size, sizeof(unsigned int)))
        return false;
    
    unsigned char* obj = new unsigned char[size];
    if(!receive(obj, size))
        return false;
    
    packet.clear();
    packet.append(obj, size);
    delete[] obj;
    
    return true;
}

bool TcpSocket::connect(const std::string& ip, unsigned short port) {
    sockaddr_in serv_addr;
    
    socket_handle = socket(AF_INET, SOCK_STREAM, 0);
    if(socket_handle < 0) {
        handleError();
        return false;
    }
 
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(port);
    
    if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0) {
        handleError();
        return false;
    }
 
    if(::connect(socket_handle, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0)
        return false;
    
    return true;
}

void TcpSocket::setBlocking(bool blocking) {
    _socketSetBlocking(socket_handle, blocking);
}

std::string TcpSocket::getIpAddress() {
    return ip_address;
}

void TcpSocket::disconnect() {
    close(socket_handle);
}

bool TcpSocket::hasDisconnected() {
    return disconnected;
}

void TcpSocket::flushPacketBuffer() {
    if(!packet_buffer.empty()) {
        send(packet_buffer.data(), (unsigned int)packet_buffer.size());
        packet_buffer.clear();
    }
}
