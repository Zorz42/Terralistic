#include "clientNetworking.hpp"

void NetworkingManager::sendPacket(sf::Packet& packet) {
    master_packet.append(packet.getData(), packet.getDataSize());
}

void NetworkingManager::checkForPackets() {
    sf::Packet packet;
    
    while(true) {
        sf::Socket::Status status = socket.receive(packet);
        if(status != sf::Socket::NotReady && status != sf::Socket::Disconnected) {
            while(!packet.endOfPacket()) {
                PacketType packet_type;
                packet >> packet_type;
                ClientPacketEvent(packet, packet_type).call();
            }
        } else
            break;
    }
}

bool NetworkingManager::establishConnection(const std::string &ip, unsigned short port) {
    if(socket.connect(ip, port) != sf::Socket::Done)
        return false;
    
    return true;
}

void NetworkingManager::disableBlocking() {
    socket.setBlocking(false);
}

void NetworkingManager::closeConnection() {
    socket.disconnect();
}

sf::Packet NetworkingManager::getPacket() {
    sf::Packet packet;
    socket.receive(packet);
    return packet;
}

std::vector<char> NetworkingManager::getData(unsigned int size) {
    std::vector<char> data(size);
    size_t received;
    socket.receive(&data[0], size, received);
    return data;
}

void NetworkingManager::flushPackets() {
    if(master_packet.getDataSize()) {
        socket.send(master_packet);
        master_packet.clear();
    }
}
