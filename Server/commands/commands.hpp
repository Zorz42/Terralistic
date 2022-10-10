#pragma once
#include <utility>

#include "serverChat.hpp"
#include "time.hpp"


class Command {
public:
    void returnInfo(ServerPlayer* sender, std::string message);
    void returnWarning(ServerPlayer* sender, std::string message);
    void returnError(ServerPlayer* sender, std::string message);
    Print* print;
    ServerTime* time;
    Blocks* blocks;
    Liquids* liquids;
    Entities* entities;
    Items* items;
    ServerPlayers* players;
    ServerChat* chat;
    Command(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print, std::string identifier, std::string usage, std::string  description) : time(time), blocks(blocks), liquids(liquids), entities(entities), items(items), players(players), chat(chat), print(print), identifier(std::move(identifier)), usage(std::move(usage)), description(std::move(description)) {}
    std::string identifier, usage, description;
    virtual bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) = 0;
};

class SetblockCommand : public Command {
public:
    SetblockCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "setBlock",
                                                                                                        "Possible invocations of this command:\n"
                                                                                                        "setBlock [x_coordinate] [y_coordinate] [block]", "place a block in world") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class SetliquidCommand : public Command {
public:
    SetliquidCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "setLiquid",
                                                                                                                          "Possible invocations of this command:\n"
                                                                                                                          "setLiquid [x_coordinate] [y_coordinate] [liquid]"
                                                                                                                          "setLiquid [x_coordinate] [y_coordinate] [liquid] [liquid_level]", "place a liquid in world") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class FillCommand : public Command {
public:
    FillCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "fill",
                                                                                                                          "Possible invocations of this command:\n"
                                                                                                                          "fill [x_coordinate_1] [y_coordinate_1] [x_coordinate_2] [y_coordinate_2] [block]", "fill a region of world with a block") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class TpCommand : public Command {
public:
    TpCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "tp",
                                                                                                  "Possible invocations of teleport command:\n"
                                                                                                  "tp [player_name] -> teleport yourself to that player\n"
                                                                                                  "tp [player_1_name] [player_2_name] -> teleport player 1 to player 2\n"
                                                                                                  "tp [x_coordinate] [y_coordinate] -> teleport yourself to that location\n"
                                                                                                  "tp [player_name] [x_coordinate] [y_coordinate] -> teleport that player to that location", "teleport players") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class GiveCommand : public Command {
public:
    GiveCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "give",
                                                                                                    "Possible invocations of this command:\n"
                                                                                                    "give [item_name] -> give 1 item of that type to yourself\n"
                                                                                                    "give [item_name] [quantity] -> give entered number of items of that type to yourself\n"
                                                                                                    "give [item_name] [quantity] [player] -> give entered number of items of that type to that player", "give items to yourself") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class SetHealthCommand : public Command {
public:
    SetHealthCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "setHealth",
                                                                                                         "Possible invocations of this command:\n"
                                                                                                         "setHealth [health] -> set your health to that number\n"
                                                                                                         "setHealth [health] [player_name] -> set that player's name to that number", "set player's health") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class HelpCommand : public Command {
    std::vector<Command*>& commands;
public:
    HelpCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print, std::vector<Command*>& commands) : commands(commands), Command(time, blocks, liquids, entities, items, players, chat, print, "help",
                                                                                                    "Possible invocations of this command:\n"
                                                                                                    "help -> list all commands\n"
                                                                                                    "help [command] -> display help for specific command\n", "display this list") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class KillCommand : public Command {
public:
    KillCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "kill",
                                                                                                         "Possible invocations of this command:\n"
                                                                                                         "kill -> kill yourself\n"
                                                                                                         "kill [player_name] -> kill someone else",
                                                                                                                           "kill a player") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class TimeCommand : public Command {
public:
    TimeCommand(ServerTime* time, Blocks* blocks, Liquids* liquids, Entities* entities, Items* items, ServerPlayers* players, ServerChat* chat, Print* print) : Command(time, blocks, liquids, entities, items, players, chat, print, "time",
                                                                                                         "Possible invocations of this command:\n"
                                                                                                         "time [time] -> set time from start of the day in seconds\n",
                                                                                                                           "set time") {}
    bool onCommand(std::vector<std::string>& args, ServerPlayer* executor) override;
};

class Commands : public ServerModule, EventListener<ServerChatEvent> {
    void onEvent(ServerChatEvent& event) override;

    Print* print;
    Blocks* blocks;
    Liquids* liquids;
    ServerPlayers* players;
    Items* items;
    Entities* entities;
    ServerChat* chat;
    
    SetblockCommand setblock_command;
    SetliquidCommand setliquid_command;
    TpCommand tp_command;
    GiveCommand give_command;
    SetHealthCommand health_command;
    HelpCommand help_command;
    FillCommand fill_command;
    KillCommand kill_command;
    TimeCommand time_command;
    std::vector<Command*> commands;

    void init() override;
    void stop() override;
    
    void startCommand(std::string message, ServerPlayer* player);
public:
    Commands(ServerTime* time, Blocks* blocks, Liquids* liquids, ServerPlayers* players, Items* items, Entities* entities, ServerChat* chat, Print* print) : blocks(blocks), liquids(liquids), players(players), items(items), entities(entities), chat(chat), print(print), setblock_command(time, blocks, liquids, entities, items, players, chat, print), setliquid_command(time, blocks, liquids, entities, items, players, chat, print), tp_command(time, blocks, liquids, entities, items, players, chat, print), give_command(time, blocks, liquids, entities, items, players, chat, print), health_command(time, blocks, liquids, entities, items, players, chat, print), help_command(time, blocks, liquids, entities, items, players, chat, print, commands), fill_command(time, blocks, liquids, entities, items, players, chat, print), kill_command(time, blocks, liquids, entities, items, players, chat, print), time_command(time, blocks, liquids, entities, items, players, chat, print) {}
};
