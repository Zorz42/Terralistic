#include <vector>
#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "configManager.hpp"
#include "fileManager.hpp"

#define PADDING 20

void MultiplayerSelector::init() {
    ConfigFile config(fileManager::getConfigPath());
    config.setDefaultStr("username", "");
    config.setDefaultStr("server ip", "");
    
    back_button.scale = 3;
    back_button.renderText("Back", {255, 255, 255});
    back_button.y = -PADDING;
    back_button.orientation = gfx::BOTTOM;
    
    join_button.scale = 3;
    join_button.renderText("Join Server", {255, 255, 255});
    join_button.y = -PADDING;
    join_button.orientation = gfx::BOTTOM;
    
    back_button.x = short((-join_button.getWidth() - back_button.getWidth() + back_button.getWidth() - PADDING) / 2);
    join_button.x = short((join_button.getWidth() + back_button.getWidth() - join_button.getWidth() + PADDING) / 2);
    
    server_ip.scale = 3;
    server_ip.orientation = gfx::CENTER;
    server_ip.setText("");
    server_ip.y = 3 * PADDING;
    server_ip.active = true;
    server_ip.setText(config.getStr("server ip"));
    server_ip.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || c == '.')
            return c;
        return '\0';
    };
    
    server_ip_title.renderText("Server IP:", {255, 255, 255});
    server_ip_title.scale = 3;
    server_ip_title.y = server_ip.y - server_ip.Sprite::getHeight() - PADDING;
    server_ip_title.orientation = gfx::CENTER;
    
    username.scale = 3;
    username.orientation = gfx::CENTER;
    username.setText("");
    username.y = server_ip_title.y - server_ip_title.getHeight() - 3 * PADDING;
    username.setText(config.getStr("username"));
    username.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_')
            return c;
        if(c == ' ')
            return '_';
        return '\0';
    };
    
    username_title.renderText("Username:", {255, 255, 255});
    username_title.scale = 3;
    username_title.y = username.y - username.Sprite::getHeight() - PADDING;
    username_title.orientation = gfx::CENTER;
    
    text_inputs = {&server_ip, &username};
}

void MultiplayerSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::Key::MOUSE_LEFT && join_button.isHovered()) || (key == gfx::Key::ENTER && can_connect))
        game(username.getText(), server_ip.getText()).run();
}

void MultiplayerSelector::render() {
    if(can_connect != (username.getText().size() >= 3 && !server_ip.getText().empty())) {
        can_connect = !can_connect;
        join_button.renderText("Join Server", {(unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100)});
        join_button.disabled = !can_connect;
    }
    join_button.render();
    back_button.render();
    server_ip_title.render();
    server_ip.render();
    username.render();
    username_title.render();
}

void MultiplayerSelector::stop() {
    ConfigFile config(fileManager::getConfigPath());
    config.setStr("username", username.getText());
    config.setStr("server ip", server_ip.getText());
}
