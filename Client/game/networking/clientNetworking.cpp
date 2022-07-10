#include "clientNetworking.hpp"

void ClientNetworking::sendPacket(Packet& packet) {
    socket.send(packet);
    socket.flushPacketBuffer();
}

void ClientNetworking::updateParallel(float frame_length) {
    Packet packet;
    
    while(socket.receive(packet)) {
        packet_count++;
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
    
    debug_menu->registerDebugLine(&tps_line);
    debug_menu->registerDebugLine(&ping_line);
    debug_menu->registerDebugLine(&packets_line);
}

void ClientNetworking::update(float frame_length) {
    if(line_refresh_timer.getTimeElapsed() > 1000) {
        line_refresh_timer.reset();
        
        packets_line.text = std::to_string(packet_count) + " packets per second";
        //if(packet_count == 0)
            //throw Exception("Connection timeout. Disconnected from server");
        packet_count = 0;
        
        tps_line.text = std::to_string(server_tps) + " TPS on server";
        if(received_ping_answer) {
            Packet ping;
            ping << ClientPacketType::PING;
            sendPacket(ping);
            received_ping_answer = false;
            ping_timer.reset();
        }
        
        //if(ping_timer.getTimeElapsed() > 60000)
            //throw Exception("Server did not respond in 60 seconds. It has likely crashed");
    }
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
    switch(event.packet_type) {
        case ServerPacketType::KICK: {
            std::string kick_message;
            event.packet >> kick_message;
            
            throw Exception("You got kicked for: " + kick_message);
        }
        case ServerPacketType::TPS: {
            event.packet >> server_tps;
            break;
        }
        case ServerPacketType::PING: {
            ping_line.text = "Ping: " + std::to_string(ping_timer.getTimeElapsed() / 1000);
            received_ping_answer = true;
        }
        default:;
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
