#pragma once
#include <SFML/Network.hpp>

typedef sf::Packet Packet;

enum class SocketStatus {Done, NotReady, Disconnected, Error, Partial};

class TcpSocket : public sf::TcpSocket {
public:
    SocketStatus receive(void* data, std::size_t size, std::size_t& received);
    SocketStatus receive(Packet& packet);
    SocketStatus connect(const std::string& ip, unsigned short port);
};

class TcpListener : public sf::TcpListener {
public:
    SocketStatus accpet(TcpSocket& socket);
};
