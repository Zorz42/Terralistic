#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>
#include <queue>
#include "packetType.hpp"
#include "events.hpp"
#include "serverModule.hpp"

class Connection {
    sf::TcpSocket* socket;
    std::queue<std::pair<sf::Packet, PacketType>> packet_buffer;
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    bool greeted = false;
    void send(sf::Packet& packet);
    void send(std::vector<char>& data);
    sf::Socket::Status receive(sf::Packet& packet);
    std::string getIpAddress();
    void pushPacket(sf::Packet& packet, PacketType type);
    bool hasPacketInBuffer();
    std::pair<sf::Packet, PacketType> getPacket();
    ~Connection();
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

class ServerNetworking : public ServerModule {
    std::vector<Connection*> connections;
    sf::TcpListener listener;
public:
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromConnections();
    
    bool accept_itself = false;
    
    EventSender<ServerConnectionWelcomeEvent> connection_welcome_event;
    EventSender<ServerNewConnectionEvent> new_connection_event;
    EventSender<ServerDisconnectEvent> disconnect_event;
};

#endif
