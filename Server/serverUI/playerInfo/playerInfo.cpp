#include "playerInfo.hpp"

void PlayerCard::render() {
    //name_text.render();
    gfx::RectShape(card_container.x + 2, card_container.y + 2, card_container.w - 4, card_container.h - 4).render(DARK_GREY);

}

PlayerCard::PlayerCard(Player* cplayer, gfx::Container* parent_cont) {
    player = cplayer;
    //name_text.loadFromSurface(gfx::textToSurface(player-
    card_container.parent_containter = parent_cont;
    card_container.orientation = gfx::TOP_LEFT;
    name_text.x = 5;
    name_text.y = 5;
}

void PlayerCard::transform(int x, int y, int w, int h) {
    card_container.x = 0.2 * x + 0.8 * card_container.x;
    card_container.y = 0.2 * y + 0.8 * card_container.y;
    card_container.w = 0.2 * w + 0.8 * card_container.w;
    card_container.h = 0.2 * h + 0.8 * card_container.h;
}


PlayerInfo::PlayerInfo(std::string resource_path): LauncherModule("player_info", std::move(resource_path)){
    min_width = 300;
    min_height = 90;
}

void PlayerInfo::init() {
    server->getNetworking()->new_connection_event.addListener(this);
    server->getNetworking()->disconnect_event.addListener(this);
}

void PlayerInfo::render() {
    gfx::RectShape(base_container.x + 2, base_container.y + 2, base_container.w - 4, base_container.h - 4).render(GREY);

    for(int i = 0; i < player_cards.size(); i++) {
        player_cards[i]->transform(2, i * 75 + (i + 1) * 2, base_container.w - 4, 75);
        player_cards[i]->render();
    }
}

void PlayerInfo::stop() {
    server->getNetworking()->new_connection_event.removeListener(this);
    server->getNetworking()->disconnect_event.removeListener(this);
}

void PlayerInfo::onEvent(ServerNewConnectionEvent &event) {
    Player* temp = server->getPlayers()->getPlayerByName(event.connection->player_name);
    player_cards.emplace_back(new PlayerCard(temp, &base_container));
}

void PlayerInfo::onEvent(ServerDisconnectEvent &event) {
    for(int i = 0; i < player_cards.size(); i++){
        if(player_cards[i]->player == server->getPlayers()->getPlayerByName(event.connection->player_name)){
            player_cards.erase(player_cards.begin() + i, player_cards.begin() + i + 1);
        }
    }
}