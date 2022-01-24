#pragma once
#include "serverChat.hpp"

class Command {
public:
    Blocks* blocks;
    Entities* entities;
    Items* items;
    ServerPlayers* players;
    Command(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : blocks(blocks), entities(entities), items(items), players(players) {}
    std::string indetifier;
    virtual void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) = 0;
};

class SetblockCommand : public Command {
public:
    SetblockCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) {indetifier = "setblock";}
    void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) override;
};

class TpCommand : public Command {
public:
    TpCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "tp";}
    void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) override;
};

class GiveCommand : public Command {
public:
    GiveCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "give";}
    void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) override;
};

class SetHealthCommand : public Command {
public:
    SetHealthCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "setHealth";}
    void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) override;
};

class HelpCommand : public Command {
public:
    HelpCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players) : Command(blocks, entities, items, players) { indetifier = "help";}
    void onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) override;
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
    SetHealthCommand health_command;
    HelpCommand help_command;
    std::vector<Command*> commands;

    void init() override;
    void stop() override;
    
    void startCommand(std::string message, ServerPlayer* player);
public:
    Commands(Blocks* blocks, ServerPlayers* players, Items* items, Entities* entities, ServerChat* chat) : blocks(blocks), players(players), items(items), entities(entities), chat(chat), setblock_command(blocks, entities, items, players), tp_command(blocks, entities, items, players), give_command(blocks, entities, items, players), health_command(blocks, entities, items, players), help_command(blocks, entities, items, players) {}
};
