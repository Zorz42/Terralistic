#pragma once
#include <string>
#include <vector>
#include "exception.hpp"

enum class SocketStatus {Done, NotReady, Disconnected, Error};

class Packet {
    std::vector<unsigned char> data;
    unsigned int read_pos = 0;
public:
    unsigned int getDataSize();
    void* getData();
    void append(void* data_ptr, unsigned int size);
    
    void clear();
    
    template<class T>
    Packet& operator<<(T obj) {
        append(&obj, sizeof(obj));
        return *this;
    }
    
    template<class T>
    Packet& operator>>(T& obj) {
        if(sizeof(obj) + read_pos > data.size())
            throw Exception("Reading out of packet data bounds.");
        
        obj = *(T*)&data[read_pos];
        read_pos += sizeof(T);
        return *this;
    }
    
    Packet& operator<<(std::string str);
    Packet& operator>>(std::string& str);
    
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
