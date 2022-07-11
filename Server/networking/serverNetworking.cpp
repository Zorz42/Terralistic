#include "serverNetworking.hpp"
#include "print.hpp"
#include "graphics.hpp"

bool Connection::hasBeenGreeted() const {
    return greeted;
}

void Connection::greet() {
    flushPacketBuffer();
    greeted = true;
}

ServerNetworking::ServerNetworking(int port) : port(port) {
    if(port < 0 || port > 65535)
        throw Exception("Port number out of range");
}

void ServerNetworking::postInit() {
    listener.listen(port);
    timer.reset();
}

void ServerNetworking::sendToEveryone(Packet& packet) {
    for(auto & connection : connections)
        if(connection->hasBeenGreeted() && connection->isConnected())
            connection->send(packet);
}

void ServerNetworking::update(float frame_length) {
    static Connection *connection = new Connection;
    while(listener.accept(*connection))
        if(!is_private || connection->getIpAddress() == "127.0.0.1") {
            connections.push_back(connection);
            connection = new Connection;
        }
    
    Packet packet;
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i]->hasBeenGreeted()) {
            if(!connections[i]->isConnected())
                removeConnection(connections[i]);
            else
                connections[i]->flushPacketBuffer();
        } else if(connections[i]->receive(packet)) {
            print::info(connections[i]->getIpAddress() + " connected (" + std::to_string(connections.size()) + " players online)");
            
            Packet time_packet;
            time_packet << WelcomePacketType::TIME << timer.getTimeElapsed();
            connections[i]->send(time_packet);
            
            ServerConnectionWelcomeEvent event(connections[i], packet);
            connection_welcome_event.call(event);
            
            Packet welcome_packet;
            welcome_packet << WelcomePacketType::WELCOME;
            connections[i]->send(welcome_packet);
            
            ServerNewConnectionEvent event2(connections[i]);
            new_connection_event.call(event2);
            
            connections[i]->greet();
        }
    }
}

void ServerNetworking::stop() {
    if(!is_private)
        while(!connections.empty())
            kickConnection(connections[0], "Server stopped!");
    
    listener.close();
}

void ServerNetworking::kickConnection(Connection* connection, const std::string& reason) {
    Packet kick_packet;
    kick_packet << ServerPacketType::KICK << reason;
    connection->send(kick_packet);
    
    removeConnection(connection);
}

void ServerNetworking::removeConnection(Connection* connection) {
    auto pos = std::find(connections.begin(), connections.end(), connection);
    if(pos == connections.end())
        throw Exception("Removed non existing connection.");
    
    ServerDisconnectEvent event(connection);
    disconnect_event.call(event);
    
    print::info(connection->getIpAddress() + " disconnected (" + std::to_string(connections.size() - 1) + " players online)");
    
    delete connection;
    connections.erase(pos);
}

const std::vector<Connection*>& ServerNetworking::getConnections() {
    return connections;
}
