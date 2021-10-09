#pragma once

#include "serverPlayers.hpp"

class ServerChatEvent {
public:
    ServerChatEvent(ServerPlayer* sender, std::string message) : sender(sender), message(message) {}
    ServerPlayer* sender;
    std::string message;
    bool cancelled = false;
};

class ServerChat : public ServerModule, EventListener<ServerPacketEvent> {
    void onEvent(ServerPacketEvent& event) override;
    ServerPlayers* players;
    ServerNetworking* networking;
    
    void init() override;
    void stop() override;
public:
    ServerChat(ServerPlayers* players, ServerNetworking* networking) : players(players), networking(networking) {}
    
    EventSender<ServerChatEvent> chat_event;
};
