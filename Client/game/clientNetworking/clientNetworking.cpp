#include "clientNetworking.hpp"

void networkingManager::sendPacket(sf::Packet& packet) {
    socket.send(packet);
}

void networkingManager::checkForPackets() {
    sf::Packet packet;
    
    while(true) {
        sf::Socket::Status status = socket.receive(packet);
        if(status != sf::Socket::NotReady && status != sf::Socket::Disconnected) {
            PacketType packet_type;
            packet >> packet_type;
            ClientPacketEvent(packet, packet_type).call();
        } else
            break;
    }
}

bool networkingManager::establishConnection(const std::string &ip, unsigned short port) {
    if(socket.connect(ip, port) != sf::Socket::Done)
        return false;
    
    return true;
}

void networkingManager::disableBlocking() {
    socket.setBlocking(false);
}

void networkingManager::closeConnection() {
    socket.disconnect();
}

sf::Packet networkingManager::getPacket() {
    sf::Packet packet;
    socket.receive(packet);
    return packet;
}
