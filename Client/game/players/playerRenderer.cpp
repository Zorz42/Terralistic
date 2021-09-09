#include "clientPlayers.hpp"

#define HEADER_MARGIN 4
#define HEADER_PADDING 2

void ClientPlayers::render(ClientPlayer& player_to_draw) {
    resource_pack->getPlayerTexture().render(2,
                                             gfx::getWindowWidth() / 2 + player_to_draw.x - blocks->view_x,
                                             gfx::getWindowHeight() / 2 + player_to_draw.y - blocks->view_y,
                                             {(short)(player_to_draw.texture_frame * PLAYER_WIDTH), 0, (unsigned short)(PLAYER_WIDTH), (unsigned short)(PLAYER_HEIGHT)},
                                             player_to_draw.flipped);
    if(player_to_draw.name != "_") {
        int header_x = gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.x + PLAYER_WIDTH - blocks->view_x,
        header_y = gfx::getWindowHeight() / 2 + player_to_draw.y - blocks->view_y - player_to_draw.name_text.getTextureHeight() - HEADER_MARGIN;
        gfx::RectShape(header_x - HEADER_PADDING, header_y - HEADER_PADDING, player_to_draw.name_text.getTextureWidth() + 2 * HEADER_PADDING, player_to_draw.name_text.getTextureHeight() + 2 * HEADER_PADDING).render(BLACK);
        player_to_draw.name_text.render(1, header_x, header_y);
    }
}
