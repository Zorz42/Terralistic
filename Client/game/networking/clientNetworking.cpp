#include "clientNetworking.hpp"

void ClientNetworking::sendPacket(Packet& packet) {
    socket.setBlocking(true);
    
    socket.send(packet);
}

void ClientNetworking::updateParallel(float frame_length) {
    Packet packet;
    
    while(true) {
        if(socket.isBlocking())
            socket.setBlocking(false);
        SocketStatus status = socket.receive(packet);
        if(status != SocketStatus::NotReady && status != SocketStatus::Disconnected) {
            while(!packet.endOfPacket()) {
                Packet sub_packet;
                int size;
                packet >> size;
                for(int i = 0; i < size; i++) {
                    unsigned char c;
                    packet >> c;
                    sub_packet << c;
                }
                
                ServerPacketType packet_type;
                sub_packet >> packet_type;
                ClientPacketEvent event(sub_packet, packet_type);
                packet_event.call(event);
            }
        } else
            break;
    }
}

void ClientNetworking::init() {
    packet_event.addListener(this);
    if(socket.connect(ip_address, port) != SocketStatus::Done)
        throw Exception("Could not connect to the server with ip " + ip_address);
}

void ClientNetworking::stop() {
    packet_event.removeListener(this);
    socket.disconnect();
}

Packet ClientNetworking::getPacket() {
    socket.setBlocking(true);
    
    Packet packet;
    socket.receive(packet);
    return packet;
}

std::vector<char> ClientNetworking::getData() {
    socket.setBlocking(true);
    
    Packet packet;
    socket.receive(packet);
    int size;
    packet >> size;
    

    std::vector<char> data(size);
    socket.receive(&data[0], size);
        
    return data;
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
        
        std::vector<char> data = getData();
        WelcomePacketEvent event(packet, type, data);
        welcome_packet_event.call(event);
    }
}
