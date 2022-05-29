#include "serverNetworking.hpp"
#include "print.hpp"
#include "graphics.hpp"

void Connection::send(Packet& packet) {
    socket->send(packet);
}

bool Connection::hasDisconnected() {
    return socket->hasDisconnected();
}

void Connection::flushPackets() {
    socket->flushPacketBuffer();
}

bool Connection::hasBeenGreeted() const {
    return greeted;
}

void Connection::greet() {
    flushPackets();
    greeted = true;
}

std::string Connection::getIpAddress() {
    return socket->getIpAddress();
}

Connection::~Connection() {
    delete socket;
}

Packet Connection::getPacket() {
    Packet result;
    if(!socket->receive(result))
        throw Exception("No packets to get");
    
    return result;
}

bool Connection::hasPacketInBuffer() {
    return socket->isPacketAvailable();
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
        if(connection->hasBeenGreeted())
            connection->send(packet);
}

void ServerNetworking::update(float frame_length) {
    static TcpSocket *socket = new TcpSocket;
    while(listener.accept(*socket))
        if(!is_private || socket->getIpAddress() == "127.0.0.1") {
            Connection* connection = new Connection(socket);
            connections.push_back(connection);
            socket = new TcpSocket;
        }
    
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i]->hasBeenGreeted()) {
            connections[i]->flushPackets();
            if(connections[i]->hasDisconnected())
                removeConnection(connections[i]);
            
        } else if(connections[i]->hasPacketInBuffer()) {
            Packet packet = connections[i]->getPacket();
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
