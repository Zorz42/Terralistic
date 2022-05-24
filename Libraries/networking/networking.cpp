#include "networking.hpp"

SocketStatus convertStatus(sf::Socket::Status status) {
    if(status == sf::Socket::Done)
        return SocketStatus::Done;
    else if(status == sf::Socket::Disconnected)
        return SocketStatus::Disconnected;
    else if(status == sf::Socket::NotReady)
        return SocketStatus::NotReady;
    return SocketStatus::Error;
}

SocketStatus TcpSocket::send(const void* data, std::size_t size) {
    std::size_t sent = 0;
    
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

SocketStatus TcpSocket::receive(void* data, std::size_t size) {
    std::size_t received = 0;
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
    sf::Socket::Status result = sf::TcpSocket::connect(ip, port);
    return convertStatus(result);
}

SocketStatus TcpListener::accept(TcpSocket& socket) {
    sf::Socket::Status result = sf::TcpListener::accept(socket);
    return convertStatus(result);
}
