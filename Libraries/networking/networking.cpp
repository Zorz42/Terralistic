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

void Packet::append(void* data_ptr, unsigned int size) {
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

Packet& Packet::operator>>(char& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(unsigned char& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(short& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        data = static_cast<Int16>(ntohs(static_cast<uint16_t>(data)));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(unsigned short& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        data = ntohs(data);
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(int& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        data = static_cast<Int32>(ntohl(static_cast<uint32_t>(data)));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(unsigned int& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        data = ntohl(data);
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(long long& data) {
    if (checkSize(sizeof(data)))
    {
        // Since ntohll is not available everywhere, we have to convert
        // to network byte order (big endian) manually
        Uint8 bytes[sizeof(data)];
        std::memcpy(bytes, &m_data[m_readPos], sizeof(data));
        data = (static_cast<Int64>(bytes[0]) << 56) |
               (static_cast<Int64>(bytes[1]) << 48) |
               (static_cast<Int64>(bytes[2]) << 40) |
               (static_cast<Int64>(bytes[3]) << 32) |
               (static_cast<Int64>(bytes[4]) << 24) |
               (static_cast<Int64>(bytes[5]) << 16) |
               (static_cast<Int64>(bytes[6]) <<  8) |
               (static_cast<Int64>(bytes[7])      );
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(unsigned long long& data) {
    if (checkSize(sizeof(data)))
    {
        // Since ntohll is not available everywhere, we have to convert
        // to network byte order (big endian) manually
        Uint8 bytes[sizeof(data)];
        std::memcpy(bytes, &m_data[m_readPos], sizeof(data));
        data = (static_cast<Uint64>(bytes[0]) << 56) |
               (static_cast<Uint64>(bytes[1]) << 48) |
               (static_cast<Uint64>(bytes[2]) << 40) |
               (static_cast<Uint64>(bytes[3]) << 32) |
               (static_cast<Uint64>(bytes[4]) << 24) |
               (static_cast<Uint64>(bytes[5]) << 16) |
               (static_cast<Uint64>(bytes[6]) <<  8) |
               (static_cast<Uint64>(bytes[7])      );
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(float& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(double& data) {
    if (checkSize(sizeof(data)))
    {
        std::memcpy(&data, &m_data[m_readPos], sizeof(data));
        m_readPos += sizeof(data);
    }

    return *this;
}

Packet& Packet::operator>>(char* data) {
    // First extract string length
    Uint32 length = 0;
    *this >> length;

    if ((length > 0) && checkSize(length))
    {
        // Then extract characters
        std::memcpy(data, &m_data[m_readPos], length);
        data[length] = '\0';

        // Update reading position
        m_readPos += length;
    }

    return *this;
}

Packet& Packet::operator>>(std::string& data) {
    // First extract string length
    Uint32 length = 0;
    *this >> length;

    data.clear();
    if ((length > 0) && checkSize(length))
    {
        // Then extract characters
        data.assign(&m_data[m_readPos], length);

        // Update reading position
        m_readPos += length;
    }

    return *this;
}

Packet& Packet::operator<<(char data){
    append(&data, sizeof(data));
    return *this;
}

Packet& Packet::operator<<(unsigned char data){
    append(&data, sizeof(data));
    return *this;
}

Packet& Packet::operator<<(short data) {
    auto toWrite = static_cast<Int16>(htons(static_cast<uint16_t>(data)));
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(unsigned short data) {
    Uint16 toWrite = htons(data);
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(int data) {
    Int32 toWrite = static_cast<Int32>(htonl(static_cast<uint32_t>(data)));
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(unsigned int data) {
    Uint32 toWrite = htonl(data);
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(long long data) {
    // Since htonll is not available everywhere, we have to convert
    // to network byte order (big endian) manually
    Uint8 toWrite[] =
    {
        static_cast<Uint8>((data >> 56) & 0xFF),
        static_cast<Uint8>((data >> 48) & 0xFF),
        static_cast<Uint8>((data >> 40) & 0xFF),
        static_cast<Uint8>((data >> 32) & 0xFF),
        static_cast<Uint8>((data >> 24) & 0xFF),
        static_cast<Uint8>((data >> 16) & 0xFF),
        static_cast<Uint8>((data >>  8) & 0xFF),
        static_cast<Uint8>((data      ) & 0xFF)
    };
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(unsigned long long data) {
    // Since htonll is not available everywhere, we have to convert
    // to network byte order (big endian) manually
    Uint8 toWrite[] =
    {
        static_cast<Uint8>((data >> 56) & 0xFF),
        static_cast<Uint8>((data >> 48) & 0xFF),
        static_cast<Uint8>((data >> 40) & 0xFF),
        static_cast<Uint8>((data >> 32) & 0xFF),
        static_cast<Uint8>((data >> 24) & 0xFF),
        static_cast<Uint8>((data >> 16) & 0xFF),
        static_cast<Uint8>((data >>  8) & 0xFF),
        static_cast<Uint8>((data      ) & 0xFF)
    };
    append(&toWrite, sizeof(toWrite));
    return *this;
}

Packet& Packet::operator<<(float data) {
    append(&data, sizeof(data));
    return *this;
}

Packet& Packet::operator<<(double data) {
    append(&data, sizeof(data));
    return *this;
}

Packet& Packet::operator<<(const char* data) {
    // First insert string length
    auto length = static_cast<Uint32>(std::strlen(data));
    *this << length;

    // Then insert characters
    append(data, length * sizeof(char));

    return *this;
}

Packet& Packet::operator<<(const std::string& data) {
    // First insert string length
    auto length = static_cast<Uint32>(data.size());
    *this << length;

    // Then insert characters
    if (length > 0)
        append(data.c_str(), length * sizeof(std::string::value_type));

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

SocketStatus TcpSocket::send(const void* data, unsigned int size) {
    unsigned int sent = 0;
    while(sent < size) {
        int curr_sent = (int)::send(socket_handle, (void*)((unsigned long)data + sent), std::min(size - sent, 65536u), 0);
        sent += curr_sent;
        
        if(curr_sent < 0)
            return getErrorStatus();
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

SocketStatus TcpSocket::receive(void* data, unsigned int size) {
    unsigned int received = 0;
    while(received < size) {
        int curr_received = (int)::read(socket_handle, (void*)((unsigned long)data + received), std::min(size - received, 65536u));
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
    
    unsigned char* data = new unsigned char[size];
    status = receive(data, size);
    if(status != SocketStatus::Done)
        return status;
    
    packet.clear();
    packet.append(data, size);
    delete[] data;
    
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

