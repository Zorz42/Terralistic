#include "networking.hpp"
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include "exception.hpp"

void socketSetBlocking(int socket_handle, bool blocking) {
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
        unsigned int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)data + sent), std::min(size - sent, 65536u), 0);
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
        unsigned int curr_received = (int)::read(socket_handle, (void*)((unsigned long)data + received), std::min(size - received, 65536u));
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
    if(status != SocketStatus::Done)
        return status;
    
    packet.clear();
    packet.append(data, size);
    delete[] data;
    
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
    socketSetBlocking(socket_handle, blocking);
}

std::string TcpSocket::getIpAddress() {
    return ip_address;
}

void TcpSocket::disconnect() {
    close(socket_handle);
}

TcpListener::TcpListener() {
    
}

TcpListener::~TcpListener() {
    
}

void TcpListener::setBlocking(bool blocking) {
    socketSetBlocking(listener_handle, blocking);
}

void TcpListener::listen(unsigned short port) {
    struct sockaddr_in address;
    int opt = 1;
    
    listener_handle = socket(AF_INET, SOCK_STREAM, 0);
    if(listener_handle == 0)
        throw Exception("Socket failed");
 
    if(setsockopt(listener_handle, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)))
        throw Exception("Setsockopt failed");
    
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(port);
 
    if(bind(listener_handle, (struct sockaddr*)&address, sizeof(address)) < 0)
        throw Exception("Cannot bind to socket");
    
    if(::listen(listener_handle, 3) < 0)
        throw Exception("Cannot listen to socket");
}

SocketStatus TcpListener::accept(TcpSocket& socket) {
    sockaddr_in address;
    int addrlen = sizeof(address);
    socket.socket_handle = ::accept(listener_handle, (struct sockaddr*)&address, (socklen_t*)&addrlen);
    if(socket.socket_handle < 0)
        return getErrorStatus();
    
    socket.ip_address = inet_ntoa(address.sin_addr);
    
    return SocketStatus::Done;
}

void TcpListener::close() {
    shutdown(listener_handle, SHUT_RDWR);
}

