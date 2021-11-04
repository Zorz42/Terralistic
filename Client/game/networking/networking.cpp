#include "clientNetworking.hpp"
#include "choiceScreen.hpp"

void ClientNetworking::sendPacket(sf::Packet& packet) {
    if(!socket.isBlocking())
        socket.setBlocking(true);
    
    socket.send(packet);
}

void ClientNetworking::update(float frame_length) {
    sf::Packet packet;
    
    while(true) {
        if(socket.isBlocking())
            socket.setBlocking(false);
        sf::Socket::Status status = socket.receive(packet);
        if(status != sf::Socket::NotReady && status != sf::Socket::Disconnected) {
            ServerPacketType packet_type;
            packet >> packet_type;
            ClientPacketEvent event(packet, packet_type);
            packet_event.call(event);
        } else
            break;
    }
}

void ClientNetworking::init() {
    packet_event.addListener(this);
    if(socket.connect(ip_address, port) != sf::Socket::Done)
        throw InvalidConnectionException();
}

void ClientNetworking::stop() {
    packet_event.removeListener(this);
    socket.disconnect();
}

sf::Packet ClientNetworking::getPacket() {
    if(!socket.isBlocking())
        socket.setBlocking(true);
    
    sf::Packet packet;
    socket.receive(packet);
    return packet;
}

std::vector<char> ClientNetworking::getData() {
    if(!socket.isBlocking())
        socket.setBlocking(true);
    
    sf::Packet packet;
    socket.receive(packet);
    int size;
    packet >> size;
    
    size_t received;
    std::vector<char> data(size);
    int bytes_received = 0;
    while(bytes_received < size) {
        socket.receive(&data[bytes_received], size - bytes_received, received);
        bytes_received += received;
    }
        
    return data;
}

void ClientNetworking::onEvent(ClientPacketEvent& event) {
    if(event.packet_type == ServerPacketType::KICK) {
        std::string kick_message;
        event.packet >> kick_message;
        
        throw KickException(kick_message);
    }
}

void ClientNetworking::postInit() {
    sf::Packet join_packet;
    join_packet << username;
    sendPacket(join_packet);
    
    while(true) {
        sf::Packet packet = getPacket();
        WelcomePacketType type;
        packet >> type;
        if(type == WelcomePacketType::WELCOME)
            break;
        
        WelcomePacketEvent event(packet, type);
        welcome_packet_event.call(event);
    }
}
