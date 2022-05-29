#pragma once
#include <queue>
#include "packetType.hpp"
#include "events.hpp"
#include "serverModule.hpp"
#include "graphics.hpp"

class Connection {
    TcpSocket* socket;
    Packet master_packet;
    std::queue<std::pair<Packet, ClientPacketType>> packet_buffer;
    bool greeted = false;
public:
    explicit Connection(TcpSocket* socket) : socket(socket) {}
    
    void send(Packet& packet);
    void sendDirectly(Packet& packet);
    
    bool hasBeenGreeted() const;
    void greet();
    
    SocketStatus receive(Packet& packet);
    
    std::string getIpAddress();
    
    void pushPacket(Packet& packet, ClientPacketType type);
    bool hasPacketInBuffer();
    std::pair<Packet, ClientPacketType> getPacket();
    
    std::string player_name;
    
    void flushPackets();
    ~Connection();
};

class ServerConnectionWelcomeEvent {
public:
    ServerConnectionWelcomeEvent(Connection* connection, Packet& client_welcome_packet) : connection(connection), client_welcome_packet(client_welcome_packet) {}
    Connection* connection;
    Packet& client_welcome_packet;
};

class ServerNewConnectionEvent {
public:
    explicit ServerNewConnectionEvent(Connection* connection) : connection(connection) {}
    Connection* connection;
};

class ServerDisconnectEvent {
public:
    explicit ServerDisconnectEvent(Connection* connection) : connection(connection) {}
    Connection* connection;
};

class ServerNetworking : public ServerModule {
    std::vector<Connection*> connections;
    TcpListener listener;
    int port;
    gfx::Timer timer;

    void postInit() override;
    void update(float frame_length) override;
    void stop() override;
    
    void removeConnection(Connection* connection);
    
public:
    explicit ServerNetworking(int port);
    
    void sendToEveryone(Packet& packet);
    void kickConnection(Connection* connection, const std::string& reason);
    
    const std::vector<Connection*>& getConnections();
    
    bool is_private = false;
    
    EventSender<ServerConnectionWelcomeEvent> connection_welcome_event;
    EventSender<ServerNewConnectionEvent> new_connection_event;
    EventSender<ServerDisconnectEvent> disconnect_event;
};
