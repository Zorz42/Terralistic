#include "clientPlayers.hpp"

#define HEADER_PADDING 4

void ClientPlayers::render(ClientPlayer& player_to_draw) {
    resource_pack->getPlayerTexture().render(2,
                                             gfx::getWindowWidth() / 2 - getPlayerWidth() / 2 + player_to_draw.x - blocks->view_x,
                                             gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + player_to_draw.y - blocks->view_y,
                                             {(short)(gfx::getTicks() / 100 % 16 * 14), 0, (unsigned short)(getPlayerWidth() / 2), (unsigned short)(getPlayerHeight() / 2)},
                                             player_to_draw.flipped);
}

void ClientPlayers::render(OtherPlayer& player_to_draw) {
    render(*(ClientPlayer*)&player_to_draw);
    player_to_draw.name_text.render(1, gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.x - blocks->view_x, gfx::getWindowHeight() / 2 - getPlayerHeight() / 2 + player_to_draw.y - blocks->view_y - player_to_draw.name_text.getTextureHeight() - HEADER_PADDING);
}

unsigned short ClientPlayers::getPlayerWidth() {
    return resource_pack->getPlayerTexture().getTextureWidth() * 2 / 16;
}

unsigned short ClientPlayers::getPlayerHeight() {
    return resource_pack->getPlayerTexture().getTextureHeight() * 2;
}
