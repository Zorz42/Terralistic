//
//  clientNetworking.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include "clientNetworking.hpp"

void networkingManager::sendPacket(sf::Packet& packet) {
    socket.send(packet);
}

void networkingManager::listenerLoop() {
    while(listener_running) {
        sf::Packet packet;
        socket.receive(packet);
        PacketType packet_type;
        packet >> packet_type;
        ClientPacketEvent(packet, packet_type).call();
    }
}

bool networkingManager::establishConnection(const std::string &ip, unsigned short port) {
    if(socket.connect(ip, port) != sf::Socket::Done)
        return false;
    listener_thread = std::thread(&networkingManager::listenerLoop, this);
    return true;
}

void networkingManager::closeConnection() {
    listener_running = false;
    listener_thread.join();
}
