#include "networking.hpp"
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include "exception.hpp"

unsigned int Packet::getDataSize() {
    return (int)data.size();
}

void* Packet::getData() {
    return data.data();
}

void Packet::append(const void* data_ptr, unsigned int size) {
    unsigned int old_size = (int)data.size();
    data.resize(old_size + size);
    std::memcpy(&data[old_size], data_ptr, size);
}

bool Packet::endOfPacket() {
    return read_pos == data.size();
}

void Packet::clear() {
    data.clear();
    read_pos = 0;
}

Packet& Packet::operator>>(char& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned char& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(short& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohs(uint32_t(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned short& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohs(obj);
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(int& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohl(uint32_t(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned int& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohl(obj);
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(long long& obj) {
    unsigned char bytes[sizeof(obj)];
    std::memcpy(bytes, &data[read_pos], sizeof(obj));
    obj = ((unsigned long long)(bytes[0]) << 56) |
           ((unsigned long long)(bytes[1]) << 48) |
           ((unsigned long long)(bytes[2]) << 40) |
           ((unsigned long long)(bytes[3]) << 32) |
           ((unsigned long long)(bytes[4]) << 24) |
           ((unsigned long long)(bytes[5]) << 16) |
           ((unsigned long long)(bytes[6]) <<  8) |
           ((unsigned long long)(bytes[7])      );
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned long long& obj) {
    unsigned char bytes[sizeof(obj)];
    std::memcpy(bytes, &data[read_pos], sizeof(obj));
    obj = ((unsigned long long)(bytes[0]) << 56) |
           ((unsigned long long)(bytes[1]) << 48) |
           ((unsigned long long)(bytes[2]) << 40) |
           ((unsigned long long)(bytes[3]) << 32) |
           ((unsigned long long)(bytes[4]) << 24) |
           ((unsigned long long)(bytes[5]) << 16) |
           ((unsigned long long)(bytes[6]) <<  8) |
           ((unsigned long long)(bytes[7])      );
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(float& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(double& obj) {
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(std::string& obj) {
    unsigned int length = 0;
    *this >> length;

    obj.clear();
    if(length > 0) {
        obj.assign(&data[read_pos], length);
        read_pos += length;
    }

    return *this;
}

Packet& Packet::operator<<(char obj){
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(unsigned char obj){
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(short obj) {
    short to_write = htons(uint16_t(obj));
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(unsigned short obj) {
    unsigned short to_write = htons(obj);
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(int obj) {
    int to_write = htonl(uint32_t(obj));
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(unsigned int obj) {
    unsigned int to_write = htonl(obj);
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(long long obj) {
    unsigned char to_write[] = {
        (unsigned char)((obj >> 56) & 0xFF),
        (unsigned char)((obj >> 48) & 0xFF),
        (unsigned char)((obj >> 40) & 0xFF),
        (unsigned char)((obj >> 32) & 0xFF),
        (unsigned char)((obj >> 24) & 0xFF),
        (unsigned char)((obj >> 16) & 0xFF),
        (unsigned char)((obj >>  8) & 0xFF),
        (unsigned char)((obj      ) & 0xFF)
    };
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(unsigned long long obj) {
    unsigned int to_write[] = {
        (unsigned char)((obj >> 56) & 0xFF),
        (unsigned char)((obj >> 48) & 0xFF),
        (unsigned char)((obj >> 40) & 0xFF),
        (unsigned char)((obj >> 32) & 0xFF),
        (unsigned char)((obj >> 24) & 0xFF),
        (unsigned char)((obj >> 16) & 0xFF),
        (unsigned char)((obj >>  8) & 0xFF),
        (unsigned char)((obj      ) & 0xFF)
    };
    append(&to_write, sizeof(to_write));
    return *this;
}

Packet& Packet::operator<<(float obj) {
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(double obj) {
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(const std::string& obj) {
    unsigned int length = (unsigned int)obj.size();
    *this << length;

    if (length > 0)
        append(obj.c_str(), length * sizeof(std::string::value_type));

    return *this;
}


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

SocketStatus TcpSocket::send(const void* obj, unsigned int size) {
    unsigned int sent = 0;
    while(sent < size) {
        int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)obj + sent), std::min(size - sent, 65536u), 0);
        sent += curr_sent;
        
        if(curr_sent < 0)
            return getErrorStatus();
    }
    
    return SocketStatus::Done;
}

SocketStatus TcpSocket::send(Packet& packet) {
    unsigned int size = (unsigned int)packet.getDataSize();
    unsigned char* obj = new unsigned char[size + 4];
    *(unsigned int*)obj = size;
    memcpy(&obj[sizeof(unsigned int)], packet.getData(), size);
    SocketStatus status = send(obj, size + 4);
    delete[] obj;
    
    return status;
}

SocketStatus TcpSocket::receive(void* obj, unsigned int size) {
    unsigned int received = 0;
    while(received < size) {
        int curr_received = (int)::read(socket_handle, (void*)((unsigned long)obj + received), std::min(size - received, 65536u));
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
    
    unsigned char* obj = new unsigned char[size];
    status = receive(obj, size);
    if(status != SocketStatus::Done)
        return status;
    
    packet.clear();
    packet.append(obj, size);
    delete[] obj;
    
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

