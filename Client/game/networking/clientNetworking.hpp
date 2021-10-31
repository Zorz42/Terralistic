#pragma once

#include "events.hpp"
#include "packetType.hpp"
#include "graphics.hpp"
#include "clientModule.hpp"

class ClientPacketEvent {
public:
    ClientPacketEvent(sf::Packet& packet, ServerPacketType packet_type) : packet(packet), packet_type(packet_type) {}
    sf::Packet& packet;
    ServerPacketType packet_type;
};

class WelcomePacketEvent {
public:
    WelcomePacketEvent(sf::Packet& packet, WelcomePacketType packet_type) : packet(packet), packet_type(packet_type) {}
    sf::Packet& packet;
    WelcomePacketType packet_type;
};

class ClientNetworking : public ClientModule, EventListener<ClientPacketEvent> {
    sf::TcpSocket socket;
    
    std::string ip_address, username;
    unsigned short port;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void postInit() override;
    void stop() override;
    void update(float frame_length) override;
public:
    ClientNetworking(const std::string& ip_address, unsigned short port, const std::string& username) : ip_address(ip_address), port(port), username(username) {}
    
    void sendPacket(sf::Packet& packet);
    std::vector<char> getData();
    sf::Packet getPacket();
    
    EventSender<ClientPacketEvent> packet_event;
    EventSender<WelcomePacketEvent> welcome_packet_event;
};
