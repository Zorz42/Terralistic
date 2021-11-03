#include "commands.hpp"
#include "print.hpp"
#include "vector"

void Commands::onEvent(ServerChatEvent& event) {
    if(event.message[0] == '/') {
        startCommand(event.message, event.sender);
        event.cancelled = true;
    }
}

void Commands::init() {
    chat->chat_event.addListener(this);
    
    commands.push_back(&tp_command);
    commands.push_back(&give_command);
    commands.push_back(&setblock_command);
}

void Commands::stop() {
    chat->chat_event.removeListener(this);
}

int formatCoord(std::string coord_str, int curr_coord) {
    int coord = 0;
    if(coord_str[0] == '~')
        coord += curr_coord;
    coord_str.erase(coord_str.begin());
    if(!coord_str.empty())
        coord += std::stoi(coord_str);
    return coord;
}

void TpCommand::onCommand(const std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() >= 2) {
        int x_coord = formatCoord(args[0], executor->getX() / 16), y_coord = formatCoord(args[1], -executor->getY() / 16 + blocks->getHeight());
        ServerPlayer* to_teleport = executor;
        if(args.size() >= 3)
            to_teleport = players->getPlayerByName(args[2]);
        y_coord = -y_coord + blocks->getHeight();
        entities->setX(to_teleport, x_coord * 16);
        entities->setY(to_teleport, y_coord * 16);
        
    }
}

void GiveCommand::onCommand(const std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() >= 1) {
        ItemTypeOld item = getItemTypeByNameOld(args[0]);
        int quantity = 1;
        if(args.size() >= 2)
            quantity = std::stoi(args[1]);
        executor->inventory.addItem(item, quantity);
    }
}

void SetblockCommand::onCommand(const std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() >= 3) {
        int x_coord = formatCoord(args[0], executor->getX() / 16), y_coord = formatCoord(args[1], -executor->getY() / 16 + blocks->getHeight());
        BlockTypeOld block = getBlockTypeByNameOld(args[2]);
        y_coord = -y_coord + blocks->getHeight();
        blocks->setBlockType(x_coord, y_coord, block);
    }
}

void Commands::startCommand(std::string message, ServerPlayer* player) {
    std::vector<std::string> args;
    size_t pos = message.find(' ');
    while (pos != std::string::npos) {
        args.push_back(message.substr(0, pos));
        message.erase(0, pos + 1);
        pos = message.find(' ');
    }
    args.push_back(message.substr(0, pos));
    
    std::string indentifier = args[0];
    args.erase(args.begin());
    indentifier.erase(indentifier.begin());

    for(Command* command : commands)
        if(command->indetifier == indentifier)
            command->onCommand(args, player);
}
