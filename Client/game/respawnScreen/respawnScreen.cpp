#include "respawnScreen.hpp"

void RespawnScreen::init() {
    back_rect.orientation = gfx::CENTER;
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_radius = BLUR;
    back_rect.shadow_intensity = SHADOW_INTENSITY;
    back_rect.border_color = BORDER_COLOR;
    back_rect.smooth_factor = 3;
    
    you_died_text.scale = 4;
    you_died_text.orientation = gfx::CENTER;
    //you_died_text.loadFromText("You Died");
    
    respawn_button.scale = 3;
    respawn_button.orientation = gfx::CENTER;
    //respawn_button.loadFromText("Respawn"); // it only works twice for some reason
    //respawn_button.loadFromText("Respawn");
    
    back_rect.setWidth(respawn_button.getWidth() + 100);
    back_rect.setY(-2 * back_rect.getHeight());
}

bool RespawnScreen::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && respawn_button.isHovered(getMouseX(), getMouseY())) {
        sf::Packet packet;
        packet << ClientPacketType::PLAYER_RESPAWN;
        networking->sendPacket(packet);
        return true;
    }
    return false;
}

void RespawnScreen::render() {
    if(players->getMainPlayer())
        first_time = false;
    is_active = !players->getMainPlayer() && !first_time;

    back_rect.setHeight(gfx::getWindowHeight());
    back_rect.setY(is_active ? 0 : -2 * back_rect.getHeight());
    you_died_text.y = back_rect.getY() - you_died_text.getHeight() / 2 - SPACING / 2;
    respawn_button.y = back_rect.getY() + respawn_button.getHeight() / 2 + SPACING / 2;
    
    float progress = float(back_rect.getY() + 2 * back_rect.getHeight()) / back_rect.getHeight() / 2;
    
    if(progress || is_active) {
        gfx::RectShape(0, 0, gfx::getWindowWidth(), gfx::getWindowHeight()).render({200, 0, 0, (unsigned char)(100 * progress)});
        back_rect.render();
        you_died_text.render();
        respawn_button.render(getMouseX(), getMouseY());
    }
}
