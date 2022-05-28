#pragma once
#include <string>
#include <vector>
#include "exception.hpp"

enum class SocketStatus {Done, NotReady, Disconnected, Error};

class Packet {
    std::vector<char> data;
    unsigned int read_pos = 0;
public:
    unsigned int getDataSize();
    void* getData();
    void append(const void* data_ptr, unsigned int size);
    
    void clear();
    
    Packet& operator>>(char& obj);
    Packet& operator>>(unsigned char& obj);
    Packet& operator>>(short& obj);
    Packet& operator>>(unsigned short& obj);
    Packet& operator>>(int& obj);
    Packet& operator>>(unsigned int& obj);
    Packet& operator>>(long long& obj);
    Packet& operator>>(unsigned long long& obj);
    Packet& operator>>(float& obj);
    Packet& operator>>(double& obj);
    Packet& operator>>(std::string& obj);
    
    Packet& operator<<(char obj);
    Packet& operator<<(unsigned char obj);
    Packet& operator<<(short obj);
    Packet& operator<<(unsigned short obj);
    Packet& operator<<(int obj);
    Packet& operator<<(unsigned int obj);
    Packet& operator<<(long long obj);
    Packet& operator<<(unsigned long long obj);
    Packet& operator<<(float obj);
    Packet& operator<<(double obj);
    Packet& operator<<(const std::string& obj);
    
    bool endOfPacket();
};

class TcpSocket {
protected:
    friend class TcpListener;
    int socket_handle;
    std::string ip_address;
public:
    SocketStatus send(const void* data, unsigned int size);
    SocketStatus send(Packet& packet);
    
    SocketStatus receive(void* data, unsigned int size);
    SocketStatus receive(Packet& packet);
    
    SocketStatus connect(const std::string& ip, unsigned short port);
    void disconnect();
    
    void setBlocking(bool blocking);
    std::string getIpAddress();
};

class TcpListener {
    int listener_handle;
public:
    void setBlocking(bool blocking);
    
    void listen(unsigned short port);
    SocketStatus accept(TcpSocket& socket);
    void close();
};
