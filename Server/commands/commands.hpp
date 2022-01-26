#pragma once
#include "serverChat.hpp"

class Command {
public:
    Blocks* blocks;
    Entities* entities;
    Items* items;
    ServerPlayers* players;
    ServerChat* chat;
    Command(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, const std::string& indentifier, const std::string& usage, const std::string& description) : blocks(blocks), entities(entities), items(items), players(players), chat(chat), indentifier(indentifier), usage(usage), description(description) {}
    std::string indentifier, usage, description;
    virtual bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) = 0;
};

class SetblockCommand : public Command {
public:
    SetblockCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat) : Command(blocks, entities, items, players, chat, "setblock",
                                                                                                        "Possible invocations of this command:\n"
                                                                                                        "setBlock [block] [x_coordinate] [y_coordinate]", "place a block in world") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class TpCommand : public Command {
public:
    TpCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat) : Command(blocks, entities, items, players, chat, "tp",
                                                                                                  "Possible invocations of teleport command:\n"
                                                                                                  "tp [player_name] -> teleport yourself to that player\n"
                                                                                                  "tp [player_1_name] [player_2_name] -> teleport player 1 to player 2\n"
                                                                                                  "tp [x_coordinate] [y_coordinate] -> teleport yourself to that location\n"
                                                                                                  "tp [player_name] [x_coordinate] [y_coordinate] -> teleport that player to that location", "teleport players") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class GiveCommand : public Command {
public:
    GiveCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat) : Command(blocks, entities, items, players, chat, "give",
                                                                                                    "Possible invocations of this command:\n"
                                                                                                    "give [item_name] -> give 1 item of that type to yourself\n"
                                                                                                    "give [item_name] [quantity] -> give entered number of items of that type to yourself\n"
                                                                                                    "give [item_name] [quantity] [player] -> give entered number of items of that type to that player", "give items to yourself") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class SetHealthCommand : public Command {
public:
    SetHealthCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat) : Command(blocks, entities, items, players, chat, "sethealth", "Possible invocations of this command:\n"
                                                                                                         "sethealth [health] -> set your health to that number\n"
                                                                                                         "sethealth [health] [player_name] -> set that player's name to that number", "set player's health") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class HelpCommand : public Command {
    std::vector<Command*>& commands;
public:
    HelpCommand(Blocks* blocks, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, std::vector<Command*>& commands) : commands(commands), Command(blocks, entities, items, players, chat, "help",
                                                                                                    "Possible invocations of this command:\n"
                                                                                                    "help -> list all commands\n"
                                                                                                    "help [command] -> display help for specific command\n", "display this list") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
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
    Commands(Blocks* blocks, ServerPlayers* players, Items* items, Entities* entities, ServerChat* chat) : blocks(blocks), players(players), items(items), entities(entities), chat(chat), setblock_command(blocks, entities, items, players, chat), tp_command(blocks, entities, items, players, chat), give_command(blocks, entities, items, players, chat), health_command(blocks, entities, items, players, chat), help_command(blocks, entities, items, players, chat, commands) {}
};
