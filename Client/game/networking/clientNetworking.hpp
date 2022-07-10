#pragma once
#include <utility>

#include "events.hpp"
#include "packetType.hpp"
#include "clientModule.hpp"
#include "debugMenu.hpp"

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
    DebugMenu* debug_menu;
    
    TcpSocket socket;
    
    std::string ip_address, username;
    int port;
    
    void onEvent(ClientPacketEvent& event) override;
    
    DebugLine tps_line, ping_line, packets_line;
    bool received_ping_answer = true;
    gfx::Timer ping_timer;
    int fps_count = 0;
    int server_tps = 0;
    int packet_count = 0;
    gfx::Timer line_refresh_timer;
    
    void init() override;
    void postInit() override;
    void stop() override;
    void update(float frame_length) override;
    void updateParallel(float frame_length) override;
public:
    ClientNetworking(DebugMenu* debug_menu, std::string ip_address, int port, std::string username) : ClientModule("ClientNetworking"), debug_menu(debug_menu), ip_address(std::move(ip_address)), port(port), username(std::move(username)) {}
    
    void sendPacket(Packet& packet);
    Packet getPacket();
    
    EventSender<ClientPacketEvent> packet_event;
    EventSender<WelcomePacketEvent> welcome_packet_event;
};
