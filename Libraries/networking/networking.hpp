#pragma once
#include <SFML/Network.hpp>

typedef sf::Packet Packet;

enum class SocketStatus {Done, NotReady, Disconnected, Error};

class TcpSocket {
    int socket_handle, file_descriptor;
    std::string ip_address;
public:
    TcpSocket();
    
    SocketStatus send(const void* data, unsigned int size);
    SocketStatus send(Packet& packet);
    
    SocketStatus receive(void* data, unsigned int size);
    SocketStatus receive(Packet& packet);
    
    SocketStatus connect(const std::string& ip, unsigned short port);
    void disconnect();
    
    void setBlocking(bool blocking);
    std::string getIpAddress();
    
    ~TcpSocket();
};

class TcpListener {
public:
    TcpListener();
    
    void setBlocking(bool blocking);
    
    void listen(unsigned short port);
    SocketStatus accept(TcpSocket& socket);
    void close();
    
    ~TcpListener();
};
