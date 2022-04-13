#include "commands.hpp"

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
    commands.push_back(&setliquid_command);
    commands.push_back(&health_command);
    commands.push_back(&help_command);
    commands.push_back(&fill_command);
    commands.push_back(&kill_command);
}

void Commands::stop() {
    chat->chat_event.removeListener(this);
}

float formatCoord(std::string coord_str, float curr_coord) {
    int coord = 0;
    if(coord_str[0] == '~') {
        coord += curr_coord;
        coord_str.erase(coord_str.begin());
    }
    if(!coord_str.empty())
        coord += std::stoi(coord_str);
    return coord;
}

bool isCoord(std::string& coord_str) {
    std::string copy = coord_str;

    if(copy[0] == '~') {
        copy.erase(0);
        if(copy.empty())
            return true;
    }

    return std::all_of(coord_str.begin(), coord_str.end(), ::isdigit);
}

bool TpCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 2 && isCoord(args[0]) && isCoord(args[1])) {
        int x = formatCoord(args[0], (float)executor->getX() / 16), y = formatCoord(args[1], -(float)executor->getY() / 16 + blocks->getHeight());
        entities->setX(executor, x * 16);
        entities->setY(executor, (-y + blocks->getHeight()) * 16);
        chat->sendChat(executor, "Teleported you to x: " + std::to_string(x) + ", y: " + std::to_string(y) + ".");
        return true;
    } else if(args.size() == 1) {
        ServerPlayer* target = players->getPlayerByName(args[0]);
        entities->setX(executor, target->getX());
        entities->setY(executor, target->getY());
        chat->sendChat(executor, "Teleported you to " + args[0] + ".");
        return true;
    } else if(args.size() == 2) {
        ServerPlayer* target1 = players->getPlayerByName(args[0]), *target2 = players->getPlayerByName(args[1]);
        entities->setX(target1, target2->getX());
        entities->setY(target1, target2->getY());
        chat->sendChat(executor, "Teleported " + args[0] + " to " + args[1] + ".");
        return true;
    } else if(args.size() == 3 && isCoord(args[1]) && isCoord(args[2])) {
        ServerPlayer* target = players->getPlayerByName(args[0]);
        int x = formatCoord(args[1], (float)executor->getX() / 16), y = formatCoord(args[2], -(float)executor->getY() / 16 + blocks->getHeight());
        entities->setX(target, x * 16);
        entities->setY(target, (-y + blocks->getHeight()) * 16);
        chat->sendChat(executor, "Teleported " + args[0] + " to x: " + std::to_string(x) + ", y: " + std::to_string(y) + ".");
        return true;
    }
    return false;
}

bool GiveCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 1) {
        executor->inventory.addItem(items->getItemTypeByName(args[0]), 1);
        chat->sendChat(executor, "Gave 1 " + args[0] + ".");
        return true;
    } else if(args.size() == 2 && std::all_of(args[1].begin(), args[1].end(), ::isdigit)) {
        executor->inventory.addItem(items->getItemTypeByName(args[0]), std::stoi(args[1]));
        chat->sendChat(executor, "Gave " + args[1] + " " + args[0] + ".");
        return true;
    } else if(args.size() == 3 && std::all_of(args[1].begin(), args[1].end(), ::isdigit)) {
        players->getPlayerByName(args[2])->inventory.addItem(items->getItemTypeByName(args[0]), std::stoi(args[1]));
        chat->sendChat(executor, "Gave " + args[2] + " " + args[1] + " " + args[0] + ".");
        return true;
    }
    return false;
}

bool SetHealthCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 1 && std::all_of(args[0].begin(), args[0].end(), ::isdigit)) {
        players->setPlayerHealth(executor, std::stoi(args[0]));
        chat->sendChat(executor, "Set your health to " + args[0] + ".");
        return true;
    } else if(args.size() == 2 && std::all_of(args[0].begin(), args[0].end(), ::isdigit)) {
        players->setPlayerHealth(players->getPlayerByName(args[1]), std::stoi(args[0]));
        chat->sendChat(executor, "Set " + args[1] + "'s health to " + args[0] + ".");
        return true;
    }
    return false;
}

bool KillCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 0) {
        players->setPlayerHealth(executor, 0);
        chat->sendChat(executor, "Killed yourself.");
        return true;
    } else if(args.size() == 1) {
        players->setPlayerHealth(players->getPlayerByName(args[1]), 0);
        chat->sendChat(executor, "Killed " + args[1] + ".");
        return true;
    }
    return false;
}

bool SetblockCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 3 && isCoord(args[0]) && isCoord(args[1])) {
        int x = formatCoord(args[0], executor->getX() / 16), y = formatCoord(args[1], -executor->getY() / 16 + blocks->getHeight());
        blocks->setBlockType(x, -y + blocks->getHeight(), blocks->getBlockTypeByName(args[2]));
        chat->sendChat(executor, "Set block on x: " + std::to_string(x) + ", y: " + std::to_string(y) + " to " + args[2] + ".");
        return true;
    }
    return false;
}

bool SetliquidCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    int level = MAX_LIQUID_LEVEL;
    if(args.size() == 4){
        level = std::stoi(args[3]);
        args.erase(args.end());
    }
    if(args.size() == 3 && isCoord(args[0]) && isCoord(args[1])) {
        int x = formatCoord(args[0], executor->getX() / 16), y = formatCoord(args[1], -executor->getY() / 16 + blocks->getHeight());
        liquids->setLiquidType(x, -y + blocks->getHeight(), liquids->getLiquidTypeByName(args[2]));
        liquids->setLiquidLevel(x, -y + blocks->getHeight(), level);
        chat->sendChat(executor, "Set liquid on x: " + std::to_string(x) + ", y: " + std::to_string(y) + " to " + args[2] + ".");
        return true;
    }
    return false;
}

bool FillCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 5 && isCoord(args[0]) && isCoord(args[1]) && isCoord(args[2]) && isCoord(args[3])) {
        int x1 = formatCoord(args[0], executor->getX() / 16), y1 = formatCoord(args[1], -executor->getY() / 16 + blocks->getHeight());
        int x2 = formatCoord(args[2], executor->getX() / 16), y2 = formatCoord(args[3], -executor->getY() / 16 + blocks->getHeight());
        if((abs(x1 - x2) + 1) * (abs(y1 - y2) + 1) > 1000){
            chat->sendChat(executor, "you exceeded the limit of 1000 blocks");
            return true;
        }
        for(int i = x1; i <= x2; i++)
            for(int j = y1; j <= y2; j++)
                blocks->setBlockType(i, -j + blocks->getHeight(), blocks->getBlockTypeByName(args[4]));
        chat->sendChat(executor, "Filled from x: " + std::to_string(x1) + ", y: " + std::to_string(y1) + " to x: " + std::to_string(x2) + ", y: " + std::to_string(y2) + "with " + args[4] + ".");
        return true;
    }
    return false;
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

    for(int i = 0; i < commands.size(); i++)
        if(commands[i]->indentifier == indentifier) {
            try {
                if(!commands[i]->onCommand(args, player))
                    chat->sendChat(player, commands[i]->usage);
            } catch(const Exception& e) {
                chat->sendChat(player, e.message);
            }
            return;
        }
    
    chat->sendChat(player, "Command \"" + indentifier + "\" not recognised. Type /help for a list of commands.");
}

bool HelpCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.empty()) {
        std::string message = "List of commands:\n";
        for(int i = 0; i < commands.size(); i++)
            message += commands[i]->indentifier + " -> " + commands[i]->description + "\n";
        chat->sendChat(executor, message);
        return true;
    } else if(args.size() == 1) {
        for(int i = 0; i < commands.size(); i++)
            if(commands[i]->indentifier == args[0]) {
                chat->sendChat(executor, commands[i]->usage);
                return true;
            }
        
        chat->sendChat(executor, "Command \"" + indentifier + "\" not found.");
        return true;
    }
    return false;
}
