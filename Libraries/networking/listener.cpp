#include "networking.hpp"
#include <arpa/inet.h>
#include <unistd.h>
#include <fcntl.h>
#include "exception.hpp"

void TcpListener::setBlocking(bool blocking) {
    _socketSetBlocking(listener_handle, blocking);
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
        return _getErrorStatus();
    
    socket.ip_address = inet_ntoa(address.sin_addr);
    
    return SocketStatus::Done;
}

void TcpListener::close() {
    shutdown(listener_handle, SHUT_RDWR);
}
