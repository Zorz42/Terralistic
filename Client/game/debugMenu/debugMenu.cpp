#include "debugMenu.hpp"
#include "iostream"

void DebugMenu::init() {
    back_rect.orientation = gfx::BOTTOM_RIGHT;
    back_rect.setY(-SPACING);
    back_rect.fill_color = BLACK;
    back_rect.border_color = BORDER_COLOR;
    back_rect.fill_color.a = TRANSPARENCY;
    back_rect.blur_intensity = BLUR;
    back_rect.smooth_factor = 3;
    
    networking->packet_event.addListener(this);
    
    debug_lines = {&fps_line, &server_tps_line, &coords_line, &packets_line};
}

void DebugMenu::update(float frame_length) {
    enabled = players->getMainPlayer() != nullptr;
    
    if(enabled) {
        fps_count++;
        if(timer.getTimeElapsed() > 1000) {
            timer.reset();
            fps_line.text = std::to_string(fps_count) + " fps";
            fps_count = 0;
            
            packets_line.text = std::to_string(packet_count) + " packets per second";
            packet_count = 0;
        }
        server_tps_line.text = std::to_string(server_tps) + " TPS on server";
        
        if(debug_menu_open) {
            static int prev_x = 0, prev_y = 0;
            int curr_x = players->getMainPlayer()->getX() / (BLOCK_WIDTH * 2), curr_y = players->getMainPlayer()->getY() / (BLOCK_WIDTH * 2);
            if(curr_x != prev_x || curr_y != prev_y) {
                prev_x = curr_x;
                prev_y = curr_y;
                coords_line.text = std::string("X: ") + std::to_string(int(players->getMainPlayer()->getX() / (BLOCK_WIDTH * 2))) + ", Y: " + std::to_string(int(blocks->getHeight() - players->getMainPlayer()->getY() / (BLOCK_WIDTH * 2)));
            }
        }
        
        for(int i = 0; i < debug_lines.size(); i++)
            debug_lines[i]->update();
    }
}

void DebugMenu::DebugLine::render(int x, int y) {
    texture.render(2, x, y);
}

int DebugMenu::DebugLine::getWidth() {
    return texture.getTextureWidth() * 2;
}

int DebugMenu::DebugLine::getHeight() {
    return texture.getTextureHeight() * 2;
}

void DebugMenu::DebugLine::update() {
    if(prev_text != text) {
        prev_text = text;
        texture.loadFromText(text);
    }
}

void DebugMenu::render() {
    int back_width = 0, back_height = 0;
    
    for(int i = 0; i < debug_lines.size(); i++) {
        back_width = std::max(debug_lines[i]->getWidth(), back_width);
        back_height += debug_lines[i]->getHeight();
    }
    
    back_rect.setWidth(back_width + SPACING);
    back_rect.setHeight(back_height + SPACING);
    back_rect.setX(debug_menu_open ? -SPACING : back_rect.getWidth() + SPACING);
    back_rect.render();
    
    int curr_y = gfx::getWindowHeight() + back_rect.getY() - back_rect.getHeight();
    for(int i = 0; i < debug_lines.size(); i++) {
        debug_lines[i]->render(gfx::getWindowWidth() + back_rect.getX() - back_rect.getWidth() + SPACING / 2, curr_y + SPACING / 2);
        curr_y += debug_lines[i]->getHeight();
    }
}

void DebugMenu::onEvent(ClientPacketEvent& event) {
    packet_count++;
    switch(event.packet_type) {
        case ServerPacketType::TPS:
            event.packet >> server_tps;
            break;
        default: break;
    }
}


bool DebugMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::M) {
        debug_menu_open = !debug_menu_open;
        return true;
    }
    return false;
}

void DebugMenu::stop() {
    networking->packet_event.removeListener(this);
}
