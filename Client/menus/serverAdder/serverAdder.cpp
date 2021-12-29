#include "serverAdder.hpp"

void ServerAdder::init() {
    back_button.scale = 3;
    back_button.loadFromText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;
    
    add_server_title.loadFromText("New server address:");
    add_server_title.scale = 3;
    add_server_title.y = SPACING;
    add_server_title.orientation = gfx::TOP;
    
    add_button.scale = 3;
    add_button.loadFromText("Add server");
    add_button.y = -SPACING;
    add_button.orientation = gfx::BOTTOM;

    back_button.x = (-add_button.getWidth() - back_button.getWidth() + back_button.getWidth() - SPACING) / 2;
    add_button.x = (add_button.getWidth() + back_button.getWidth() - add_button.getWidth() + SPACING) / 2;

    server_ip_input.scale = 3;
    server_ip_input.orientation = gfx::CENTER;
    server_ip_input.setText("");
    server_ip_input.active = true;
    server_ip_input.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-' || c == '_' || c == ':' || c == '.')
            return c;
        return '\0';
    };
    
    server_ip_input.def_color.a = TRANSPARENCY;
    
    text_inputs = {&server_ip_input};
}

bool ServerAdder::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered(getMouseX(), getMouseY())) {
        returnFromScene();
        return true;
    } else if((key == gfx::Key::MOUSE_LEFT && add_button.isHovered(getMouseX(), getMouseY())) || (key == gfx::Key::ENTER && can_add)) {
        server_ip = server_ip_input.getText();
        returnFromScene();
        return true;
    }
    return false;
}

void ServerAdder::render() {
    menu_back->setBackWidth(server_ip_input.getWidth() + 100);
    menu_back->renderBack();
    if(can_add != !server_ip_input.getText().empty()) {
        can_add = !can_add;
        add_button.loadFromText("Add server", {(unsigned char)(can_add ? WHITE.r : GREY.r), (unsigned char)(can_add ? WHITE.g : GREY.g), (unsigned char)(can_add ? WHITE.b : GREY.b)});
        add_button.disabled = !can_add;
    }
    add_button.render(getMouseX(), getMouseY());
    back_button.render(getMouseX(), getMouseY());
    add_server_title.render();
    server_ip_input.render(getMouseX(), getMouseY());
}
