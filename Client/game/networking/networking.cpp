#include "clientNetworking.hpp"
#include "choiceScreen.hpp"

void NetworkingManager::sendPacket(sf::Packet& packet) {
    sf::Socket::Status status = sf::Socket::Partial;
    while(status == sf::Socket::Partial)
        status = socket.send(packet);
}

void NetworkingManager::update(float frame_length) {
    sf::Packet packet;
    
    while(true) {
        sf::Socket::Status status = socket.receive(packet);
        if(status != sf::Socket::NotReady && status != sf::Socket::Disconnected) {
            PacketType packet_type;
            packet >> packet_type;
            ClientPacketEvent event(packet, packet_type);
            packet_event.call(event);
        } else
            break;
    }
}

void NetworkingManager::init() {
    if(socket.connect(ip_address, port) != sf::Socket::Done) {
        GameErrorEvent error_event("Could not connect to the server!");
        game_error_event.call(error_event);
    }
}

void NetworkingManager::stop() {
    socket.disconnect();
}

sf::Packet NetworkingManager::getPacket() {
    sf::Packet packet;
    socket.receive(packet);
    return packet;
}

std::vector<char> NetworkingManager::getData() {
    int size;
    std::size_t temp;
    socket.receive((char*)&size, sizeof(int), temp);
    
    std::vector<char> data(size);
    int bytes_received = 0;
    size_t received;
    while(bytes_received < size) {
        socket.receive(&data[bytes_received], size, received);
        bytes_received += received;
    }
    return data;
}

void NetworkingManager::onEvent(ClientPacketEvent& event) {
    if(event.packet_type == PacketType::KICK) {
        std::string kick_message;
        event.packet >> kick_message;
        
        GameErrorEvent error_event(kick_message);
        game_error_event.call(error_event);
    }
}

void NetworkingManager::postInit() {
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
    
    socket.setBlocking(false);
}
