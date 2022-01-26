#include "commands.hpp"
#include "commandMessages.hpp"

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

void TpCommand::onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) {
    for(int i = 0; i < arg_types.size(); i++)
        if(arg_types.at(i) == 'N')
            arg_types.at(i) = 'C';
    if(arg_types == "S"){
        arg_types = "SS";
        args.insert(args.begin(), executor->name);
    }
    if(arg_types == "SS"){
        try {
            ServerPlayer* curr_player = players->getPlayerByName(args[1]);
            args.erase(args.end());
            args.push_back(std::to_string((float)curr_player->getX() / 16));
            args.push_back(std::to_string(-(float)curr_player->getY() / 16 + blocks->getHeight()));
            arg_types = "SCC";
        }catch(Exception& e) {
            playerNotFound(args[1], executor);
            return;
        }
    }
    if(arg_types == "CC"){
        args.insert(args.begin(), executor->name);
        arg_types = "SCC";
    }
    if(arg_types == "SCC"){
        try {
            ServerPlayer* to_teleport = players->getPlayerByName(args[0]);
            float x = formatCoord(args[1], (float)executor->getX() / 16);
            float y = formatCoord(args[2], -(float)executor->getY() / 16 + blocks->getHeight());
            successfulTP(to_teleport->name, args[1] + " " + args[2], executor);
            y = -y + blocks->getHeight();
            entities->setX(executor, x * 16);
            entities->setY(executor, y * 16);
            return;
        }catch (Exception& e){
            playerNotFound(args[0], executor);
            return;
        }
    }

    argumentsIncorrect("tp", executor);
}

void GiveCommand::onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) {
    if(arg_types == "I"){
        arg_types = "SI";
        args.insert(args.begin(), executor->name);
    }
    if(arg_types == "SI"){
        arg_types = "SIN";
        args.emplace_back("1");
    }
    if(arg_types == "IN"){
        arg_types = "SIN";
        args.insert(args.begin(), executor->name);
    }
    if(arg_types == "SIN") {
        ServerPlayer *reciever;
        ItemType *item;
        try {
            reciever = players->getPlayerByName(args[0]);
            item = items->getItemTypeByName(args[1].substr(5, args[1].size() - 5));
            reciever->inventory.addItem(item, std::stoi(args[2]));
        } catch (Exception &e) {
            if (e.message == "Could not find player by name")
                playerNotFound(args[0], executor);
            else
                itemNotFound(args[1], executor);
        }
        return;
    }
    argumentsIncorrect("give", executor);
}

void SetHealthCommand::onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) {
    if(arg_types == "N")
        executor->setPlayerHealth(std::stoi(args[0]));
    else if(arg_types == "SN"){
        ServerPlayer* curr_player;
        try {
            curr_player = players->getPlayerByName(args[0]);
            curr_player->setPlayerHealth(std::stoi(args[1]));
        }catch(Exception& e) {
            playerNotFound(args[0], executor);
        }
    }
}

void SetblockCommand::onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) {
    for(int i = 0; i < arg_types.size(); i++)
        if(arg_types.at(i) == 'N')
            arg_types.at(i) = 'C';
    if(arg_types == "CCB") {
        int x_coord = formatCoord(args[0], (float)executor->getX() / 16), y_coord = formatCoord(args[1], -(float)executor->getY() / 16 + blocks->getHeight());
        BlockType* block = blocks->getBlockTypeByName(args[2]);
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

    std::string arg_types;
    for(auto & arg : args){
        if(arg.size() > 5 && arg.substr(0, 5) == "Item:"){
            arg_types += 'I';//item
        }else if(arg.substr(0, 1) == "~"){
            arg_types += 'C';//coordinate
        }else if(arg.size() > 6 && arg.substr(0, 6) == "Block:"){
            arg_types += 'B';//block
        }else if(arg.size() > 7 && arg.substr(0, 7) == "Liquid:"){
            arg_types += 'L';//liquid
        }else if(std::all_of(arg.begin(), arg.end(), ::isdigit)){
            arg_types += 'N';//number
        }else{
            arg_types += 'S';//string
        }
    }

    for(int i = 0; i < commands.size(); i++)
        if(commands[i]->indetifier == indentifier) {
            commands[i]->onCommand(args, arg_types, player);
            return;
        }
    sf::Packet error_message;
    error_message << ServerPacketType::CHAT << "Command not recognised. Type /help for a list of commands.";
    player->getConnection()->send(error_message);
}

void HelpCommand::onCommand(std::vector<std::string>& args, std::string arg_types, ServerPlayer* executor) {
    if(args.empty()){
        sf::Packet help_message;
        std::string message = "List of commands:\n"
                              "help -> display this list\n"
                              "tp -> teleport players\n"
                              "give -> give items to yourself\n"
                              "setHealth -> set player's health\n"
                              "setBlock -> place a block in world\n";
        help_message << ServerPacketType::CHAT << message;
        executor->getConnection()->send(help_message);
    }else if(args.size() == 1){
        if(args[0] == "tp"){
            sf::Packet help_message;
            help_message << ServerPacketType::CHAT << "Possible invocations of teleport command:\n"
                                                      "tp [player_name] -> teleport yourself to that player\n"
                                                      "tp [player_1_name] [player_2_name] -> teleport player 1 to player 2\n"
                                                      "tp [x_coordinate] [y_coordinate] -> teleport yourself to that location\n"
                                                      "tp [player_name] [x_coordinate] [y_coordinate] -> teleport that player to that location";
            executor->getConnection()->send(help_message);
        }else if(args[0] == "give"){
            sf::Packet help_message;
            help_message << ServerPacketType::CHAT << "Possible invocations of this command:\n"
                                                      "give Item:[item_name] -> give 1 item of that type to yourself\n"
                                                      "give [player] Item:[item_name] -> give 1 item of that type to that player\n"
                                                      "give Item:[item_name] [quantity] -> give entered number of items of that type to yourself\n"
                                                      "give [player] Item:]item_name] [quantity] -> give entered number of items of that type to that player";
            executor->getConnection()->send(help_message);
        }else if(args[0] == "setHealth"){
            sf::Packet help_message;
            help_message << ServerPacketType::CHAT << "Possible invocations of this command:\n"
                                                      "setHealth [health] -> set your health to that number\n"
                                                      "setHealth [player_name] [health] -> set that player's name to that number";
            executor->getConnection()->send(help_message);
        }else if(args[0] == "setBlock"){
            sf::Packet help_message;
            help_message << ServerPacketType::CHAT << "Possible invocations of this command:\n"
                                                      "setBlock [x_coordinate] [y_coordinate] Block:[block]";
            executor->getConnection()->send(help_message);
        }
    }else{
        sf::Packet error_message;
        error_message << ServerPacketType::CHAT << "Arguments incorrect. \nUse /help to display a list of commands or\n"
                                                   "/help [command_identifier] to display a specific command's help menu";
        executor->getConnection()->send(error_message);
    }
}


































