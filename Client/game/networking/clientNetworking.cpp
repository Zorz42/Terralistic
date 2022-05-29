#include "clientNetworking.hpp"

void ClientNetworking::sendPacket(Packet& packet) {
    socket.send(packet);
    socket.flushPacketBuffer();
}

void ClientNetworking::updateParallel(float frame_length) {
    Packet packet;
    
    while(socket.receive(packet)) {
        ServerPacketType packet_type;
        packet >> packet_type;
        ClientPacketEvent event(packet, packet_type);
        packet_event.call(event);
    }
}

void ClientNetworking::init() {
    packet_event.addListener(this);
    if(!socket.connect(ip_address, port))
        throw Exception("Could not connect to the server with ip " + ip_address);
}

void ClientNetworking::stop() {
    packet_event.removeListener(this);
    socket.disconnect();
}

Packet ClientNetworking::getPacket() {
    Packet packet;
    while(!socket.receive(packet))
        gfx::sleep(1);
    return packet;
}

void ClientNetworking::onEvent(ClientPacketEvent& event) {
    if(event.packet_type == ServerPacketType::KICK) {
        std::string kick_message;
        event.packet >> kick_message;
        
        throw Exception("You got kicked for: " + kick_message);
    }
}

void ClientNetworking::postInit() {
    Packet join_packet;
    join_packet << username;
    sendPacket(join_packet);
    
    while(true) {
        Packet packet = getPacket();
        WelcomePacketType type;
        packet >> type;
        if(type == WelcomePacketType::WELCOME)
            break;
        
        WelcomePacketEvent event(packet, type);
        welcome_packet_event.call(event);
    }
}
