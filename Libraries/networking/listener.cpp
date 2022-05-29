#include "networking.hpp"
#include "exception.hpp"
#ifdef WIN32
#include <winsock.h>
#else
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#endif

void TcpListener::handleError() {
    if(errno == EAGAIN || errno == EINPROGRESS)
        return;

    switch(errno) {
        case EWOULDBLOCK:  return;
        case ECONNABORTED: return;
        case ECONNRESET:   return;
        case ETIMEDOUT:    return;
        case ENETRESET:    return;
        case ENOTCONN:     return;
        case EPIPE:        return;
        default:           throw Exception("Listener error");
    }
}

void TcpListener::listen(unsigned short port) {
    struct sockaddr_in address;
    int opt = 1;
    
    listener_handle = socket(AF_INET, SOCK_STREAM, 0);
    if(listener_handle == 0)
        throw Exception("Socket failed");

    if(setsockopt(listener_handle, SOL_SOCKET, SO_REUSEADDR, (const char*)&opt, sizeof(opt)))
        throw Exception("Setsockopt failed");
    
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(port);
 
    if(bind(listener_handle, (struct sockaddr*)&address, sizeof(address)) < 0)
        throw Exception("Cannot bind to socket");
    
    if(::listen(listener_handle, 3) < 0)
        throw Exception("Cannot listen to socket");
    
    _socketDisableBlocking(listener_handle);
}

bool TcpListener::accept(TcpSocket& socket) {
    sockaddr_in address;
    int addrlen = sizeof(address);
    socket.socket_handle = ::accept(listener_handle, (struct sockaddr*)&address, (int*)&addrlen);
    if(socket.socket_handle < 0) {
        handleError();
        return false;
    }
    
    socket.ip_address = inet_ntoa(address.sin_addr);
    
    return true;
}

void TcpListener::close() {
#ifdef WIN32
    closesocket(listener_handle);
#else
    shutdown(listener_handle, SHUT_RDWR);
#endif
}
