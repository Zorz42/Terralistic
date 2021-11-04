#pragma once

#include "serverPlayers.hpp"
#include "string"
#include "worldGenerator.hpp"
#include "entities.hpp"
#include "serverChat.hpp"

class Command {
public:
    Blocks* blocks;
    Entities* entities;
    Items* items;
    ServerPlayers* players;
    Command(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : blocks(blocks), entities(entities), items(items), players(players) {}
    std::string indetifier;
    virtual void onCommand(const std::vector<std::string>& args, ServerPlayer* executor) = 0;
};

class SetblockCommand : public Command {
public:
    SetblockCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "setblock"; }
    void onCommand(const std::vector<std::string>& args, ServerPlayer* executor) override;
};

class TpCommand : public Command {
public:
    TpCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "tp"; }
    void onCommand(const std::vector<std::string>& args, ServerPlayer* executor) override;
};

class GiveCommand : public Command {
public:
    GiveCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "give"; }
    void onCommand(const std::vector<std::string>& args, ServerPlayer* executor) override;
};

class Commands : public ServerModule, EventListener<ServerChatEvent> {
    void onEvent(ServerChatEvent& event) override;
    
    Blocks* blocks;
    ServerPlayers* players;
    Items* items;
    Entities* entities;
    ServerChat* chat;
    
    SetblockCommand setblock_command;
    TpCommand tp_command;
    GiveCommand give_command;
    std::vector<Command*> commands;
    
    void init() override;
    void stop() override;
public:
    Commands(Blocks* blocks, ServerPlayers* players, Items* items, Entities* entities, ServerChat* chat) : blocks(blocks), players(players), items(items), entities(entities), chat(chat), setblock_command(blocks, entities, items, players), tp_command(blocks, entities, items, players), give_command(blocks, entities, items, players) {}
    
    void startCommand(std::string message, ServerPlayer* player);
};
