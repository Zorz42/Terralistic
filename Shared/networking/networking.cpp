#include "networking.hpp"

SocketStatus convertStatus(sf::Socket::Status status) {
    if(status == sf::Socket::Done)
        return SocketStatus::Done;
    else if(status == sf::Socket::Disconnected)
        return SocketStatus::Disconnected;
    else if(status == sf::Socket::NotReady)
        return SocketStatus::NotReady;
    else if(status == sf::Socket::Partial)
        return SocketStatus::Partial;
    return SocketStatus::Error;
}

SocketStatus TcpSocket::send(const void* data, std::size_t size, std::size_t& sent) {
    sf::Socket::Status result = sf::TcpSocket::send(data, size, sent);
    return convertStatus(result);
}

SocketStatus TcpSocket::send(const void* data, std::size_t size) {
    std::size_t unused;
    return send(data, size, unused);
}

SocketStatus TcpSocket::send(Packet& packet) {
    sf::Socket::Status result = sf::TcpSocket::send(packet);
    return convertStatus(result);
}

SocketStatus TcpSocket::receive(void* data, std::size_t size, std::size_t& received) {
    sf::Socket::Status result = sf::TcpSocket::receive(data, size, received);
    return convertStatus(result);
}

SocketStatus TcpSocket::receive(Packet& packet) {
    sf::Socket::Status result = sf::TcpSocket::receive(packet);
    return convertStatus(result);
}

SocketStatus TcpSocket::connect(const std::string& ip, unsigned short port) {
    sf::Socket::Status result = sf::TcpSocket::connect(ip, port);
    return convertStatus(result);
}

SocketStatus TcpListener::accept(TcpSocket& socket) {
    sf::Socket::Status result = sf::TcpListener::accept(socket);
    return convertStatus(result);
}
