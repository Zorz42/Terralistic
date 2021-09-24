#pragma once

#include "serverPlayers.hpp"
#include "string"
#include "worldGenerator.hpp"
#include "serverEntities.hpp"

class Commands {
    ServerBlocks* blocks;
    Players* players;
    ServerItems* items;
    ServerEntities* entities;
public:
    Commands(ServerBlocks* blocks, Players* players, ServerItems* items, ServerEntities* entities) : blocks(blocks), players(players), items(items), entities(entities){}
    void startCommand(std::string message, std::string player);
    void changeBlock(int x, int y, std::string type);
    void format(std::vector<std::string>& message, const std::string& player);
};