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

SocketStatus _getErrorStatus() {
    if(errno == EAGAIN || errno == EINPROGRESS)
        return SocketStatus::NotReady;

    switch(errno) {
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

SocketStatus TcpSocket::send(const void* obj, unsigned int size) {
    unsigned int sent = 0;
    while(sent < size) {
        int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)obj + sent), std::min(size - sent, 65536u), 0);
        sent += curr_sent;
        
        if(curr_sent < 0)
            return _getErrorStatus();
    }
    
    return SocketStatus::Done;
}

SocketStatus TcpSocket::send(Packet& packet) {
    unsigned int size = (unsigned int)packet.getDataSize();
    unsigned char* obj = new unsigned char[size + 4];
    *(unsigned int*)obj = size;
    memcpy(&obj[sizeof(unsigned int)], packet.getData(), size);
    SocketStatus status = send(obj, size + 4);
    delete[] obj;
    
    return status;
}

SocketStatus TcpSocket::receive(void* obj, unsigned int size) {
    unsigned int received = 0;
    while(received < size) {
        int curr_received = (int)::read(socket_handle, (void*)((unsigned long)obj + received), std::min(size - received, 65536u));
        received += curr_received;
        
        if(curr_received == 0)
            return SocketStatus::Disconnected;
        else if(curr_received < 0)
            return _getErrorStatus();
    }
    
    return SocketStatus::Done;
}

SocketStatus TcpSocket::receive(Packet& packet) {
    unsigned int size;
    SocketStatus status = receive(&size, sizeof(unsigned int));
    if(status != SocketStatus::Done)
        return status;
    
    unsigned char* obj = new unsigned char[size];
    status = receive(obj, size);
    if(status != SocketStatus::Done)
        return status;
    
    packet.clear();
    packet.append(obj, size);
    delete[] obj;
    
    return SocketStatus::Done;
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
 
    if(::connect(socket_handle, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0)
        return SocketStatus::Error;
    
    return SocketStatus::Done;
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
