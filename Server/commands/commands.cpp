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
    commands.push_back(&health_command);
    commands.push_back(&help_command);
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

bool TpCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 2 && (std::all_of(args[0].begin(), args[0].end(), ::isdigit) || args[0].substr(0, 1) == "~") && (std::all_of(args[1].begin(), args[1].end(), ::isdigit) || args[0].substr(0, 1) == "~")) {
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
    } else if(args.size() == 3 && (std::all_of(args[1].begin(), args[1].end(), ::isdigit) || args[1].substr(0, 1) == "~") && (std::all_of(args[2].begin(), args[2].end(), ::isdigit) || args[2].substr(0, 1) == "~")) {
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
        executor->setPlayerHealth(std::stoi(args[0]));
        chat->sendChat(executor, "Set your health to " + args[0] + ".");
        return true;
    } else if(args.size() == 2 && std::all_of(args[0].begin(), args[0].end(), ::isdigit)) {
        players->getPlayerByName(args[1])->setPlayerHealth(std::stoi(args[0]));
        chat->sendChat(executor, "Set " + args[1] + "'s health to " + args[0] + ".");
        return true;
    }
    return false;
}

bool SetblockCommand::onCommand(std::vector<std::string>& args, ServerPlayer* executor) {
    if(args.size() == 3 && std::all_of(args[1].begin(), args[1].end(), ::isdigit) && std::all_of(args[2].begin(), args[2].end(), ::isdigit)) {
        blocks->setBlockType(std::stoi(args[1]), std::stoi(args[2]), blocks->getBlockTypeByName(args[0]));
        chat->sendChat(executor, "Set block on x: " + args[1] + ", y: " + args[2] + " to " + args[0] + ".");
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
            } catch(Exception& e) {
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
