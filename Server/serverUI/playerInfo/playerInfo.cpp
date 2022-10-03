#include "playerInfo.hpp"

void PlayerCard::render() {

}


PlayerInfo::PlayerInfo(std::string resource_path): LauncherModule("player_info", std::move(resource_path)){
    min_width = 300;
    min_height = 90;
}

void PlayerInfo::init() {

}

void PlayerInfo::render() {

    gfx::RectShape(base_container.x + 2, base_container.y + 2, base_container.w - 4, base_container.h - 4).render(GREY);

    for(auto& card : players)
        card.render();
}
