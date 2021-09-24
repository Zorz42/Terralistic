#include "commands.hpp"
#include "print.hpp"
#include "vector"

void Commands::startCommand(std::string message, std::string player) {
    std::vector<std::string> seperated;

    {
        size_t pos = message.find(' ');
        while (pos != std::string::npos) {
            seperated.push_back(message.substr(0, pos));
            message.erase(0, pos + 1);
            pos = message.find(' ');
        }
        seperated.push_back(message.substr(0, pos));
    }

    format(seperated, player);

    if(seperated[0] == "/setBlock")
        changeBlock(std::stoi(seperated[1]),std::stoi(seperated[2]), seperated[3]);
}

void Commands::changeBlock(int x, int y, std::string type) {
    BlockType block;
    //if(type.at(0) >= 0 && type.at(0) <= 9)
        block = (BlockType) std::stoi(type);
    //else
        //block = blocks->

    blocks->getBlock(x, blocks->getHeight() - y).setType(block);
    blocks->getBlock(x, blocks->getHeight() - y).update();
}

void Commands::format(std::vector<std::string>& message, const std::string& player) {
    for(int i = 1; i < message.size(); i++){
        int block_x;
        if(message[i].at(0) == '~') {
            block_x = players->getPlayerByName(player)->getX() / 16;
            if(message[i].size() > 1) {
                message[i].erase(0);
                block_x += std::stoi(message[i]);
            }
        }else
            block_x = std::stoi(message[i]);
        print::info(std::to_string(block_x));
        message[i] = std::to_string(block_x);
    }
}