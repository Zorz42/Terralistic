#include "playerInfo.hpp"
#define CARD_HEIGHT 75

void PlayerCard::render() {
    //name_text.render();
    background.render();
}

PlayerCard::PlayerCard(Player* cplayer, gfx::Container* parent_cont) {
    player = cplayer;
    card_container.parent_containter = parent_cont;
    card_container.orientation = gfx::TOP_LEFT;
    name_text.x = 5;
    name_text.y = 5;
    background.parent_containter = &card_container;
    background.fill_color = DARK_GREY;
}

void PlayerCard::transform(int x, int y, int w, int h) {
    card_container.x = 0.2 * x + 0.8 * card_container.x;
    card_container.y = 0.2 * y + 0.8 * card_container.y;
    card_container.w = 0.2 * w + 0.8 * card_container.w;
    card_container.h = 0.2 * h + 0.8 * card_container.h;

    background.w = w;
}

void PlayerCard::initTransform(int x, int y, int w, int h) {
    card_container.x = x;
    card_container.y = y;
    card_container.w = w;
    card_container.h = h;

    background.x = 0;
    background.y = 0;
    background.w = w;
    background.h = h;
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
        player_cards[i]->transform(6, i * CARD_HEIGHT + (i + 1) * 4 + 4, base_container.w - 12, CARD_HEIGHT);
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
    player_cards[player_cards.size() - 1]->initTransform(6, (player_cards.size() - 1) * CARD_HEIGHT + player_cards.size() * 4 + 2, base_container.w - 12, CARD_HEIGHT);
}

void PlayerInfo::onEvent(ServerDisconnectEvent &event) {
    for(int i = 0; i < player_cards.size(); i++){
        if(player_cards[i]->player == server->getPlayers()->getPlayerByName(event.connection->player_name)){
            player_cards.erase(player_cards.begin() + i, player_cards.begin() + i + 1);
        }
    }
}
