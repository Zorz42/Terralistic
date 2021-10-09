#pragma once

#include "serverPlayers.hpp"
#include "string"
#include "worldGenerator.hpp"
#include "entities.hpp"
#include "serverChat.hpp"

class Commands : public ServerModule, EventListener<ServerChatEvent> {
    void onEvent(ServerChatEvent& event) override;
    
    Blocks* blocks;
    ServerPlayers* players;
    Items* items;
    Entities* entities;
    ServerChat* chat;
    
    void init() override;
    void stop() override;
public:
    Commands(Blocks* blocks, ServerPlayers* players, Items* items, Entities* entities, ServerChat* chat) : blocks(blocks), players(players), items(items), entities(entities), chat(chat) {}
    
    void startCommand(std::string message, ServerPlayer* player);
    void changeBlock(std::vector<std::string>& message);
    void formatLocation(std::vector<std::string>& message, ServerPlayer* player, unsigned char start_format);
    void formatBlockType(std::string& type);
    void formatItemType(std::string& type);
    void teleport(ServerPlayer*, int x, int y);
    void giveItem(std::vector<std::string>& message, ServerPlayer* player);
};
