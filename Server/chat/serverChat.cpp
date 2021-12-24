#include "serverChat.hpp"
#include "print.hpp"

void ServerChat::onEvent(ServerPacketEvent &event) {
    if(event.packet_type == ClientPacketType::CHAT) {
        std::string message;
        event.packet >> message;
        
        ServerChatEvent event_(event.player, message);
        chat_event.call(event_);
        
        if(event_.cancelled)
            return;
        
        std::string chat_format = (event.player->name == "_" ? "Protagonist" : event.player->name) + ": " + message;
        print::info(chat_format);
        
        sf::Packet chat_packet;
        chat_packet << ServerPacketType::CHAT << chat_format;
        networking->sendToEveryone(chat_packet);
    }
}

void ServerChat::preInit() {
    players->packet_event.addListener(this);
}

void ServerChat::stop() {
    players->packet_event.removeListener(this);
}

