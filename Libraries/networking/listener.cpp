#include "networking.hpp"
#include "exception.hpp"
#ifdef WIN32
#include <winsock.h>
#else
#include <arpa/inet.h>
#include <cstring>
#endif

void TcpListener::handleError() {
    if(errno == EAGAIN || errno == EINPROGRESS)
        return;

    switch(errno) {
        case EWOULDBLOCK:
        case ECONNABORTED:
        case ECONNRESET:
        case ETIMEDOUT:
        case ENETRESET:
        case ENOTCONN:
        case EPIPE:        return;
        default:           throw Exception("Listener error");
    }
}

void TcpListener::listen(unsigned short port) {
    sockaddr_in address{};
    
    int listener_handle = socket(AF_INET, SOCK_STREAM, 0);
    if(listener_handle == 0)
        throw Exception("Socket failed");
    listener_socket.create(listener_handle, "127.0.0.1");

    std::memset(&address, 0, sizeof(address));
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(port);

#ifdef __APPLE__
    address.sin_len = sizeof(address);
#endif
 
    if(bind(listener_handle, (struct sockaddr*)&address, sizeof(address)) < 0)
        throw Exception("Cannot bind to socket");
    
    if(::listen(listener_handle, SOMAXCONN) < 0)
        throw Exception("Cannot listen to socket");
    
    _socketDisableBlocking(listener_handle);
}

bool TcpListener::accept(TcpSocket& socket) const {
    sockaddr_in address{};
    int addrlen = sizeof(address);
#ifdef WIN32
    int socket_handle = ::accept(listener_socket.socket_handle, (struct sockaddr*)&address, (int*)&addrlen);
#else
    int socket_handle = ::accept(listener_socket.socket_handle, (struct sockaddr*)&address, (socklen_t *)&addrlen);
#endif
    if(socket_handle < 0) {
        handleError();
        return false;
    }

    socket.create(socket_handle, inet_ntoa(address.sin_addr));

    return true;
}

void TcpListener::close() const {
#ifdef WIN32
    closesocket(listener_socket.socket_handle);
#else
    shutdown(listener_socket.socket_handle, SHUT_RDWR);
#endif
}
