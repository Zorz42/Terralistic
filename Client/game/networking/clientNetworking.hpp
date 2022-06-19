#pragma once
#include <utility>

#include "events.hpp"
#include "packetType.hpp"
#include "clientModule.hpp"

class ClientPacketEvent {
public:
    ClientPacketEvent(Packet& packet, ServerPacketType packet_type) : packet(packet), packet_type(packet_type) {}
    Packet& packet;
    ServerPacketType packet_type;
};

class WelcomePacketEvent {
public:
    WelcomePacketEvent(Packet& packet, WelcomePacketType packet_type) : packet(packet), packet_type(packet_type) {}
    Packet& packet;
    WelcomePacketType packet_type;
};

class ClientNetworking : public ClientModule, EventListener<ClientPacketEvent> {
    TcpSocket socket;
    
    std::string ip_address, username;
    int port;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void postInit() override;
    void stop() override;
    void updateParallel(float frame_length) override;
public:
    ClientNetworking(std::string  ip_address, int port, std::string  username) : ip_address(std::move(ip_address)), port(port), username(std::move(username)) {}
    
    void sendPacket(Packet& packet);
    Packet getPacket();
    
    EventSender<ClientPacketEvent> packet_event;
    EventSender<WelcomePacketEvent> welcome_packet_event;
};
