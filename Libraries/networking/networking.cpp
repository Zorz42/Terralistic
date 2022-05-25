#include "networking.hpp"
#include <arpa/inet.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
SocketStatus getSocketStatus() {
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

/*SocketStatus TcpSocket::send(const void* data, unsigned int size) {
    unsigned int sent = 0;
    
    while(sent < size) {
        std::size_t curr_sent;
        sf::Socket::Status result = sf::TcpSocket::send(data, size - sent, curr_sent);
        if(result == sf::Socket::Disconnected)
            return SocketStatus::Disconnected;
        else if(result == sf::Socket::Error)
            return SocketStatus::Error;
        else if(result == sf::Socket::NotReady)
            return SocketStatus::NotReady;
        sent += curr_sent;
    }
    
    return SocketStatus::Done;
}*/

SocketStatus TcpSocket::send(Packet& packet) {
    unsigned int size = (unsigned int)packet.getDataSize();
    unsigned char* data = new unsigned char[size + 4];
    *(unsigned int*)data = size;
    memcpy(&data[sizeof(unsigned int)], packet.getData(), size);
    SocketStatus status = send(data, size + 4);
    delete[] data;
    
    return status;
}

/*SocketStatus TcpSocket::receive(void* data, unsigned int size) {
 unsigned int received = 0;
    while(received < size) {
        std::size_t curr_received;
        sf::Socket::Status result = sf::TcpSocket::receive(data, size - received, curr_received);
        if(result == sf::Socket::Disconnected)
            return SocketStatus::Disconnected;
        else if(result == sf::Socket::Error)
            return SocketStatus::Error;
        else if(result == sf::Socket::NotReady)
            return SocketStatus::NotReady;
        received += curr_received;
    }
    return SocketStatus::Done;
}*/

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
    int file_descriptor;
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

/*SocketStatus TcpListener::accept(TcpSocket& socket) {
    sf::Socket::Status result = sf::TcpListener::accept(socket);
    return convertStatus(result);
}*/
