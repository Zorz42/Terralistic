#include "commandMessages.hpp"

void playerNotFound(const std::string& player_name, ServerPlayer* executor){
    sf::Packet error_message;
    error_message << ServerPacketType::CHAT << "Player with name " + player_name + " does not exist";
    executor->getConnection()->send(error_message);
}

void itemNotFound(const std::string& item_name, ServerPlayer* executor){
    sf::Packet error_message;
    error_message << ServerPacketType::CHAT << "Item with name " + item_name.substr(5, item_name.size() - 5) + " does not exist";
    executor->getConnection()->send(error_message);
}

void argumentsIncorrect(const std::string& command_name, ServerPlayer* executor){
    sf::Packet error_message;
    error_message << ServerPacketType::CHAT << "Arguments incorrect. Use \"/help " + command_name + "\" for a list of arguments";
    executor->getConnection()->send(error_message);
}

void successfulTP(const std::string& teleported_player, const std::string& destination, ServerPlayer* executor){
    sf::Packet feedback_message;
    feedback_message << ServerPacketType::CHAT << "Successfully teleported " + teleported_player + " to " + destination;
    executor->getConnection()->send(feedback_message);
}