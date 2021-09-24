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
    std::string name;
public:
    Commands(ServerBlocks* blocks, Players* players, ServerItems* items, ServerEntities* entities) : blocks(blocks), players(players), items(items), entities(entities){}
    void StartCommand(std::string, std::string);
    void changeBlock(std::string, std::string, BlockType);
};