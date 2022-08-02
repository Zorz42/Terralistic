#pragma once
#include <queue>
#include "packetType.hpp"
#include "events.hpp"
#include "serverModule.hpp"
#include "graphics.hpp"

class Connection : public TcpSocket {
    bool greeted = false;
public:
    bool hasBeenGreeted() const;
    void greet();
    
    std::string player_name;
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
    int port;
    
    EventSender<ServerConnectionWelcomeEvent> connection_welcome_event;
    EventSender<ServerNewConnectionEvent> new_connection_event;
    EventSender<ServerDisconnectEvent> disconnect_event;
};
