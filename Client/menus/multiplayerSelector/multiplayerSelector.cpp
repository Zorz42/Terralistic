#include "platform_folders.h"
#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "choiceScreen.hpp"

#define TOP_HEIGHT (title.getHeight() + 2 * SPACING)
#define BOTTOM_HEIGHT (back_button.getHeight() + 2 * SPACING)

void MenuServer::render(int position, int mouse_x, int mouse_y) {
    int render_x = gfx::getWindowWidth() / 2 - 400 + SPACING, render_y = y - position, render_width = 800 - 2 * SPACING, render_height = 116 + 2 * SPACING;
    
    gfx::Color back_color = BLACK;
    back_color.a = 100;
    gfx::RectShape(render_x, render_y, render_width, render_height).render(back_color);
    
    icon.render(1, render_x + SPACING, render_y + SPACING);
    ip_texture.render(3, render_x + 2 * SPACING + icon.getTextureWidth(), render_y + SPACING * 1.5);
    
    join_button.x = render_x + 2 * SPACING + icon.getTextureWidth();
    join_button.y = render_y + render_height - join_button.getHeight() - SPACING;
    join_button.render(mouse_x, mouse_y);
    
    remove_button.x = render_x + 3 * SPACING + icon.getTextureWidth() + join_button.getWidth();
    remove_button.y = render_y + render_height - join_button.getHeight() - SPACING;
    remove_button.render(mouse_x, mouse_y);
}

void MultiplayerSelector::refresh() {
    for(int i = 0; i < servers.size(); i++)
        delete servers[i];
    servers.clear();
    
    scroll_limit = 0;
    
    for(int i = 0; i < server_ips.size(); i++) {
        servers.push_back(new MenuServer);
        MenuServer* server = servers.back();
        
        server->y = scroll_limit + TOP_HEIGHT;
        
        server->icon.loadFromResources("world_icon.png");
        
        server->ip = server_ips[i];
        server->ip_texture.loadFromText(server_ips[i]);

        server->join_button.loadFromResources("join_button.png");
        server->join_button.scale = 3;
        server->join_button.margin = 5;
        
        server->remove_button.loadFromResources("remove_button.png");
        server->remove_button.scale = 3;
        server->remove_button.margin = 5;
        
        scroll_limit += 116 + SPACING * 3;
    }
}

void MultiplayerSelector::init() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setDefaultStr("username", "");
    config.setDefaultStr("servers", "");
    
    title.scale = 3;
    title.loadFromText("Select a server to join!");
    title.y = SPACING;
    title.orientation = gfx::TOP;

    back_button.scale = 3;
    back_button.loadFromText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;

    new_button.scale = 3;
    new_button.loadFromText("Add");
    new_button.y = -SPACING;
    new_button.orientation = gfx::BOTTOM;
    
    top_rect.orientation = gfx::TOP;
    top_rect.setHeight(TOP_HEIGHT);
    
    bottom_rect.orientation = gfx::BOTTOM;
    bottom_rect.setHeight(BOTTOM_HEIGHT);
    bottom_rect.fill_color.a = TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = SHADOW_INTENSITY;
    bottom_rect.blur_intensity = BLUR;
    
    std::string servers_str = config.getStr("servers"), curr_serv;
    for(int i = 0; i < servers_str.size(); i++) {
        if(servers_str[i] == ' ') {
            server_ips.push_back(curr_serv);
            curr_serv.clear();
        } else
            curr_serv.push_back(servers_str[i]);
    }
    
    refresh();
    
/*[](char c, int length) {
        if((c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-' || c == '_' || c == ':' || c == '.')
            return c;
        return '\0';
    };*/
    
   /* [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_')
            return c;
        if(c == ' ')
            return '_';
        return '\0';
    };*/
}

bool MultiplayerSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        /*else if(new_button.isHovered(getMouseX(), getMouseY())) {
            std::vector<std::string> worlds_names;
            for(int i = 0; i < worlds.size(); i++)
                worlds_names.push_back(worlds[i]->name);
            WorldCreator world_creator(worlds_names, menu_back, settings);
            switchToScene(world_creator);
            refresh();
        }*/
        else
            for(int i = 0; i < servers.size(); i++) {
                if(servers[i]->join_button.isHovered(getMouseX(), getMouseY())) {
                    //startPrivateWorld(sago::getDataHome() + "/Terralistic/Worlds/" + worlds[i]->name + ".world", menu_back, settings, false);
                } else if(servers[i]->remove_button.isHovered(getMouseX(), getMouseY())) {
                    std::string result;
                    if(getKeyState(gfx::Key::SHIFT))
                        result = "Yes";
                    else {
                        ChoiceScreen choice_screen(menu_back, "Do you want to remove " + servers[i]->ip + "?", {"Yes", "No"}, &result);
                        switchToScene(choice_screen);
                    }

                    if(result == "Yes") {
                        server_ips.erase(server_ips.begin() + i);
                        refresh();
                    }
                    break;
                }
            }
        return true;
    }
    return false;
}

void MultiplayerSelector::onMouseScroll(int distance) {
    position -= distance * 8;
    if(position < 0)
        position = 0;
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position > scroll_limit_)
        position = scroll_limit_ > 0 ? scroll_limit_ : 0;
}

void MultiplayerSelector::render() {
    menu_back->setBackWidth(800);
    menu_back->renderBack();
    
    bool hoverable = getMouseY() > TOP_HEIGHT && getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(int i = 0; i < servers.size(); i++) {
        servers[i]->join_button.disabled = !hoverable;
        servers[i]->remove_button.disabled = !hoverable;
    }

    for(int i = 0; i < servers.size(); i++)
        servers[i]->render(position, getMouseX(), getMouseY());

    top_rect.setWidth(menu_back->getBackWidth());
    top_rect_visibility += ((position ? 1.f : 0.f) - top_rect_visibility) / 20;
    if(top_rect_visibility < 0.01f)
        top_rect_visibility = 0;
    if(top_rect_visibility > 0.99f)
        top_rect_visibility = 1;
    top_rect.fill_color.a = top_rect_visibility * TRANSPARENCY / 2;
    top_rect.blur_intensity = top_rect_visibility * BLUR;
    top_rect.shadow_intensity = top_rect_visibility * SHADOW_INTENSITY;
    if(top_rect_visibility)
        top_rect.render();
    
    bottom_rect.setWidth(menu_back->getBackWidth());
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(scroll_limit_ > 0)
        bottom_rect.render();

    title.render();
    back_button.render(getMouseX(), getMouseY());
    
    new_button.x = menu_back->getBackWidth() / 2 - SPACING - new_button.getWidth() / 2;
    new_button.render(getMouseX(), getMouseY());
}

void MultiplayerSelector::stop() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    std::string servers_str;
    for(int i = 0; i < server_ips.size(); i++) {
        servers_str += server_ips[i];
        servers_str.push_back(' ');
    }
    config.setStr("servers", servers_str);
}
