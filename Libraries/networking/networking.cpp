#include "networking.hpp"
#include <arpa/inet.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdexcept>

void fdSetBlocking(int file_descriptor, bool blocking) {
#ifdef _WIN32
    unsigned long mode = blocking ? 0 : 1;
    if(ioctlsocket(fd, FIONBIO, &mode) != 0)
        throw std::runtime_error("Could not set socket to blocking");
#else
    int flags = fcntl(file_descriptor, F_GETFL, 0);
    if(flags == -1)
        throw std::runtime_error("Could not set socket to blocking");
    flags = blocking ? (flags & ~O_NONBLOCK) : (flags | O_NONBLOCK);
    if(fcntl(file_descriptor, F_SETFL, flags) != 0)
        throw std::runtime_error("Could not set socket to blocking");
#endif
}

TcpSocket::TcpSocket() {
    
}

TcpSocket::~TcpSocket() {
    
}

SocketStatus getErrorStatus() {
    if ((errno == EAGAIN) || (errno == EINPROGRESS))
        return SocketStatus::NotReady;

    switch (errno) {
        case EWOULDBLOCK:  return SocketStatus::NotReady;
        case ECONNABORTED: return SocketStatus::Disconnected;
        case ECONNRESET:   return SocketStatus::Disconnected;
        case ETIMEDOUT:    return SocketStatus::Disconnected;
        case ENETRESET:    return SocketStatus::Disconnected;
        case ENOTCONN:     return SocketStatus::Disconnected;
        case EPIPE:        return SocketStatus::Disconnected;
        default:           return SocketStatus::Error;
    }
}

SocketStatus TcpSocket::send(const void* data, unsigned int size) {
    unsigned int sent = 0;
    while(sent < size) {
        unsigned int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)data + sent), size - sent, 0);
        sent += curr_sent;
        
        if(curr_sent < 0)
            return getErrorStatus();
    }
    
    return SocketStatus::Done;
}

SocketStatus TcpSocket::send(Packet& packet) {
    unsigned int size = (unsigned int)packet.getDataSize();
    unsigned char* data = new unsigned char[size + 4];
    *(unsigned int*)data = size;
    memcpy(&data[sizeof(unsigned int)], packet.getData(), size);
    SocketStatus status = send(data, size + 4);
    delete[] data;
    
    return status;
}

SocketStatus TcpSocket::receive(void* data, unsigned int size) {
    unsigned int received = 0;
    while(received < size) {
        unsigned int curr_received = (int)::send(socket_handle, (void*)((unsigned long)data + received), size - received, 0);
        received += curr_received;
        
        if(curr_received == 0)
            return SocketStatus::Disconnected;
        else if(curr_received < 0)
            return getErrorStatus();
    }
    
    return SocketStatus::Done;
}

SocketStatus TcpSocket::receive(Packet& packet) {
    unsigned int size;
    SocketStatus status = receive(&size, sizeof(unsigned int));
    if(status != SocketStatus::Done)
        return status;
    unsigned char* data = new unsigned char[size];
    status = receive(data, size);
    packet.clear();
    packet.append(data, size);
    delete[] data;
    
    return status;
}

SocketStatus TcpSocket::connect(const std::string& ip, unsigned short port) {
    sockaddr_in serv_addr;
    
    socket_handle = socket(AF_INET, SOCK_STREAM, 0);
    if(socket_handle < 0)
        return SocketStatus::Error;
 
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(port);
    
    if(inet_pton(AF_INET, ip.c_str(), &serv_addr.sin_addr) <= 0)
        return SocketStatus::Error;
 
    file_descriptor = ::connect(socket_handle, (struct sockaddr*)&serv_addr, sizeof(serv_addr));
    if(file_descriptor < 0)
        return SocketStatus::Error;
    
    return SocketStatus::Done;
}

void TcpSocket::setBlocking(bool blocking) {
    fdSetBlocking(file_descriptor, blocking);
}

std::string TcpSocket::getIpAddress() {
    return ip_address;
}

void TcpSocket::disconnect() {
    close(file_descriptor);
}

TcpListener::TcpListener() {
    
}

TcpListener::~TcpListener() {
    
}

void TcpListener::setBlocking(bool blocking) { // implement
    
}

void TcpListener::listen(unsigned short port) { // implement
    
}

SocketStatus TcpListener::accept(TcpSocket& socket) { // implement
    return SocketStatus::Done;
}

void TcpListener::close() { // implement
    
}

