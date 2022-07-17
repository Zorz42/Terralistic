#include <cstring>
#include "networking.hpp"

#ifdef WIN32
#include <winsock.h>
#else
#include <arpa/inet.h>
#endif

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

void Packet::checkReadSize(unsigned int read_size) {
    if(read_pos + read_size > data.size())
        throw PacketError("Reading packet out of bounds");
}

Packet& Packet::operator>>(bool& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(char& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned char& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(short& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohs(uint32_t(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned short& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohs(obj);
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(int& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohl(uint32_t(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned int& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    obj = ntohl(obj);
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(long long& obj) {
    checkReadSize(sizeof(obj));
    unsigned char bytes[sizeof(obj)];
    std::memcpy(bytes, &data[read_pos], sizeof(obj));
    obj = ((long long)(bytes[0]) << 56) |
           ((long long)(bytes[1]) << 48) |
           ((long long)(bytes[2]) << 40) |
           ((long long)(bytes[3]) << 32) |
           ((long long)(bytes[4]) << 24) |
           ((long long)(bytes[5]) << 16) |
           ((long long)(bytes[6]) <<  8) |
           ((long long)(bytes[7])      );
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(unsigned long long& obj) {
    checkReadSize(sizeof(obj));
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
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(double& obj) {
    checkReadSize(sizeof(obj));
    std::memcpy(&obj, &data[read_pos], sizeof(obj));
    read_pos += sizeof(obj);

    return *this;
}

Packet& Packet::operator>>(std::string& obj) {
    unsigned int length = 0;
    *this >> length;
    
    checkReadSize(length);
    obj.clear();
    if(length > 0) {
        obj.assign(&data[read_pos], length);
        read_pos += length;
    }

    return *this;
}

Packet& Packet::operator>>(std::vector<char>& obj) {
    unsigned int length;
    *this >> length;
    checkReadSize(length);
    obj.resize(length);
    std::memcpy(obj.data(), &data[read_pos], length);
    read_pos += length;
    
    return *this;
}

Packet& Packet::operator>>(std::vector<unsigned char>& obj) {
    unsigned int length;
    *this >> length;
    checkReadSize(length);
    obj.resize(length);
    std::memcpy(obj.data(), &data[read_pos], length);
    read_pos += length;
    
    return *this;
}

Packet& Packet::operator<<(bool obj) {
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(char obj) {
    append(&obj, sizeof(obj));
    return *this;
}

Packet& Packet::operator<<(unsigned char obj) {
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

    append(obj.c_str(), length);

    return *this;
}

Packet& Packet::operator<<(const std::vector<char>& obj) {
    *this << (unsigned int)obj.size();
    append(obj.data(), (unsigned int)obj.size());
    
    return *this;
}

Packet& Packet::operator<<(const std::vector<unsigned char>& obj) {
    *this << (unsigned int)obj.size();
    append(obj.data(), (unsigned int)obj.size());
    
    return *this;
}
