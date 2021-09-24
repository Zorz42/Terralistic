#include "commands.hpp"
#include "print.hpp"
#include "vector"

void Commands::StartCommand(std::string message, std::string player) {
    std::vector<std::string> seperated;
    {
        size_t pos = message.find(' ');
        while (pos != std::string::npos) {
            seperated.push_back(message.substr(0, pos));
            message.erase(0, pos + 1);
            pos = message.find(' ');
        }
        seperated.push_back(message.substr(0, pos));
        for(const std::string& part : seperated)
            print::info(part + "\n");
    }
    name = player;

    if(seperated[0] == "/setBlock")
        changeBlock(seperated[1], seperated[2], (BlockType)std::stoi(seperated[3]));
}

void Commands::changeBlock(std::string x, std::string y, BlockType type) {
    unsigned int block_x, block_y;
    if(x.at(0) == '~') {
        block_x = players->getPlayerByName(name)->getX() / 16;
        if(x.size() > 1) {
            x.erase(0);
            block_x += std::stoi(x);
        }
    }else
        block_x = std::stoi(x);

    /*if(y.at(0) == '~') {
        block_y = players->getPlayerByName(name)->getY();
        if(y.size() > 1)
            block_y += std::stoi(y.substr(1, y.size() - 1));
    }else*/
        block_y = std::stoi(y);

    blocks->getBlock(block_x, blocks->getHeight() - block_y).setType(type);
    blocks->getBlock(block_x, blocks->getHeight() - block_y).update();
}
