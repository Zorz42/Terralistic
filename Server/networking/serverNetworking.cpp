#include "serverNetworking.hpp"
#include "print.hpp"
#include "graphics.hpp"

void Connection::send(Packet& packet) {
    master_packet << (int)packet.getDataSize();
    master_packet.append(packet.getData(), packet.getDataSize());
}

void Connection::sendDirectly(Packet& packet) {
    socket->send(packet);
}

void Connection::flushPackets() {
    if(master_packet.getDataSize() != 0) {
        socket->send(master_packet);
        master_packet.clear();
    }
}

SocketStatus Connection::receive(Packet& packet) {
    return socket->receive(packet);
}

bool Connection::hasBeenGreeted() const {
    return greeted;
}

void Connection::greet() {
    socket->setBlocking(false);
    greeted = true;
}

std::string Connection::getIpAddress() {
    return socket->getIpAddress();
}

Connection::~Connection() {
    delete socket;
}

void Connection::send(const std::vector<char>& data) {
    Packet packet;
    packet << (int)data.size();
    sendDirectly(packet);
    
    socket->send(&data[0], (int)data.size());
}

void Connection::pushPacket(Packet& packet, ClientPacketType type) {
    packet_buffer.push({packet, type});
}

bool Connection::hasPacketInBuffer() {
    return !packet_buffer.empty();
}

std::pair<Packet, ClientPacketType> Connection::getPacket() {
    auto result = packet_buffer.front();
    packet_buffer.pop();
    return result;
}

ServerNetworking::ServerNetworking(int port) : port(port) {
    if(port < 0 || port > 65535)
        throw Exception("Port number out of range");
}

void ServerNetworking::postInit() {
    listener.listen(port);
    listener.setBlocking(false);
    timer.reset();
}

void ServerNetworking::sendToEveryone(Packet& packet) {
    for(auto & connection : connections)
        if(connection->hasBeenGreeted())
            connection->send(packet);
}

void ServerNetworking::update(float frame_length) {
    static TcpSocket *socket = new TcpSocket;
    while(listener.accept(*socket) != SocketStatus::NotReady)
        if(!is_private || socket->getIpAddress() == "127.0.0.1") {
            Connection* connection = new Connection(socket);
            connections.push_back(connection);
            socket = new TcpSocket;
        }
    
    Packet packet;
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i]->hasBeenGreeted()) {
            connections[i]->flushPackets();
            while(true) {
                SocketStatus status = connections[i]->receive(packet);
                
                if(status == SocketStatus::NotReady)
                    break;
                else if(status == SocketStatus::Disconnected) {
                    removeConnection(connections[i]);
                    
                    break;
                } else if(status == SocketStatus::Done) {
                    int packet_type;
                    packet >> packet_type;
                    
                    connections[i]->pushPacket(packet, (ClientPacketType)packet_type);
                }
            }
        } else if(connections[i]->receive(packet) != SocketStatus::NotReady) {
            Packet time_packet;
            time_packet << WelcomePacketType::TIME << timer.getTimeElapsed();
            connections[i]->sendDirectly(time_packet);
            connections[i]->send(std::vector<char>(1));
            
            ServerConnectionWelcomeEvent event(connections[i], packet);
            connection_welcome_event.call(event);
            
            Packet welcome_packet;
            welcome_packet << WelcomePacketType::WELCOME;
            connections[i]->sendDirectly(welcome_packet);
            
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
    
    print::info(connection->getIpAddress() + " disconnected (" + std::to_string(connections.size()) + " players online)");
    
    delete connection;
    connections.erase(pos);
}

const std::vector<Connection*>& ServerNetworking::getConnections() {
    return connections;
}
