#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>
#include "packetType.hpp"
#include "events.hpp"

class Connection {
    sf::TcpSocket* socket;
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    bool greeted = false;
    void send(sf::Packet& packet);
    void send(std::vector<char>& data);
    sf::Socket::Status receive(sf::Packet& packet);
    std::string getIpAddress();
    void freeSocket();
};

class ServerPacketEvent {
public:
    ServerPacketEvent(Connection* connection, sf::Packet& packet) : connection(connection), packet(packet) {}
    Connection* connection;
    sf::Packet& packet;
};

class ServerConnectionWelcomeEvent {
public:
    ServerConnectionWelcomeEvent(Connection* connection, sf::Packet& client_welcome_packet) : connection(connection), client_welcome_packet(client_welcome_packet) {}
    Connection* connection;
    sf::Packet& client_welcome_packet;
};

class ServerNewConnectionEvent {
public:
    ServerNewConnectionEvent(Connection* connection) : connection(connection) {}
    Connection* connection;
};

class ServerDisconnectEvent {
public:
    ServerDisconnectEvent(Connection* connection) : connection(connection) {}
    Connection* connection;
};

class ServerNetworkingManager {
    std::vector<Connection*> connections;
    sf::TcpListener listener;
public:
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    
    bool accept_itself = false;
    
    EventSender<ServerPacketEvent> packet_event;
    EventSender<ServerConnectionWelcomeEvent> connection_welcome_event;
    EventSender<ServerNewConnectionEvent> new_connection_event;
    EventSender<ServerDisconnectEvent> disconnect_event;
};

#endif
