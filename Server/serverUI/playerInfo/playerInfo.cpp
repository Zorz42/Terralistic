#include "playerInfo.hpp"

PlayerInfo::PlayerInfo(std::string resource_path): LauncherModule("player_info", std::move(resource_path)){
    min_width = 300;
    min_height = 90;
}

void PlayerInfo::init() {

}

void PlayerInfo::render() {

}