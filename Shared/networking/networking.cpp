#include "networking.hpp"

SocketStatus TcpSocket::receive(void* data, std::size_t size, std::size_t& received) {
    sf::Socket::Status result = sf::TcpSocket::receive(data, size, received);
    if(result == sf::Socket::Done)
        return SocketStatus::Done;
    else if(result == sf::Socket::Disconnected)
        return SocketStatus::Disconnected;
    else if(result == sf::Socket::NotReady)
        return SocketStatus::NotReady;
    else if(result == sf::Socket::Partial)
        return SocketStatus::Partial;
    return SocketStatus::Error;
}

SocketStatus TcpSocket::receive(Packet& packet) {
    sf::Socket::Status result = sf::TcpSocket::receive(packet);
    if(result == sf::Socket::Done)
        return SocketStatus::Done;
    else if(result == sf::Socket::Disconnected)
        return SocketStatus::Disconnected;
    else if(result == sf::Socket::NotReady)
        return SocketStatus::NotReady;
    else if(result == sf::Socket::Partial)
        return SocketStatus::Partial;
    return SocketStatus::Error;
}

SocketStatus TcpSocket::connect(const std::string& ip, unsigned short port) {
    sf::Socket::Status result = sf::TcpSocket::connect(ip, port);
    if(result == sf::Socket::Done)
        return SocketStatus::Done;
    else if(result == sf::Socket::Disconnected)
        return SocketStatus::Disconnected;
    else if(result == sf::Socket::NotReady)
        return SocketStatus::NotReady;
    else if(result == sf::Socket::Partial)
        return SocketStatus::Partial;
    return SocketStatus::Error;
}
