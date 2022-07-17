#pragma once
#include <string>
#include <vector>
#include <queue>
#include "exception.hpp"

class Packet {
    friend class TcpSocket;
    friend class TcpListener;
    std::vector<char> data;
    unsigned int read_pos = 0;
    void checkReadSize(unsigned int read_size);
    
    void append(const void* data_ptr, unsigned int size);
    
    void* getData();
    unsigned int getDataSize();
    
public:
    Packet& operator>>(bool& obj);
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
    Packet& operator>>(std::vector<char>& obj);
    Packet& operator>>(std::vector<unsigned char>& obj);
    
    Packet& operator<<(bool obj);
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
    Packet& operator<<(const std::vector<char>& obj);
    Packet& operator<<(const std::vector<unsigned char>& obj);
};

class TcpSocket {
    friend class TcpListener;
    std::vector<char> packet_buffer_out;
    std::queue<Packet> packet_buffer_in;
    int socket_handle;
    std::string ip_address;
    
    void send(const void* data, unsigned int size);
    bool receive(void* data, unsigned int size);
    
    void handleError();
    
    bool receivePacket();
    
    bool connected = false;

    void create(int handle, const std::string& address);
public:    
    void send(Packet& packet);
    bool receive(Packet& packet);
    void flushPacketBuffer();
    
    bool connect(const std::string& ip, unsigned short port);
    std::string getIpAddress();
    void disconnect();
    
    bool isConnected() const;
};

class TcpListener {
    TcpSocket listener_socket;
    
    static void handleError();
public:
    void listen(unsigned short port);
    bool accept(TcpSocket& socket) const;
    void close() const;
};

void _socketDisableBlocking(int socket_handle);

EXCEPTION_TYPE(SocketError)
EXCEPTION_TYPE(BindError)
EXCEPTION_TYPE(ListenError)
EXCEPTION_TYPE(AlreadyConnectedError)
EXCEPTION_TYPE(NotConnectedError)
