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
    bool greeted = false;
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    
    void send(sf::Packet& packet);
    void send(std::vector<char>& data);
    
    bool hasBeenGreeted();
    void greet();
    
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
    unsigned short port;

    void init() override;
    void update(float frame_length) override;
    void stop() override;
    
    void removeConnection(Connection* connection);
    
public:
    ServerNetworking(unsigned short port) : port(port) {}
    
    void sendToEveryone(sf::Packet& packet);
    void kickConnection(Connection* connection, const std::string& reason);
    
    bool is_private = false;
    
    EventSender<ServerConnectionWelcomeEvent> connection_welcome_event;
    EventSender<ServerNewConnectionEvent> new_connection_event;
    EventSender<ServerDisconnectEvent> disconnect_event;
};

#endif
