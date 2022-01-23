#pragma once
#include "commands.hpp"

void playerNotFound(const std::string& player_name, ServerPlayer* executor);

void itemNotFound(const std::string& item_name, ServerPlayer* executor);

void argumentsIncorrect(const std::string& command_name, ServerPlayer* executor);

void successfulTP(const std::string& teleported_player, const std::string& destination, ServerPlayer* executor);