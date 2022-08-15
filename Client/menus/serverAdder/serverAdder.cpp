#include "serverAdder.hpp"

void ServerAdder::init() {
    back_button.setScale(3);
    back_button.loadFromSurface(gfx::textToSurface("Back"));
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;
    
    add_server_title.loadFromSurface(gfx::textToSurface("Add a new server"));
    add_server_title.setScale(3);
    add_server_title.y = SPACING;
    add_server_title.orientation = gfx::TOP;
    
    add_button.setScale(3);
    add_button.loadFromSurface(gfx::textToSurface("Add server"));
    add_button.y = -SPACING;
    add_button.orientation = gfx::BOTTOM;

    back_button.x = (-add_button.getWidth() - back_button.getWidth() + back_button.getWidth() - SPACING) / 2;
    add_button.x = (add_button.getWidth() + back_button.getWidth() - add_button.getWidth() + SPACING) / 2;

    server_ip_input.setScale(3);
    server_ip_input.orientation = gfx::CENTER;
    server_ip_input.setText("");
    server_ip_input.active = false;
    server_ip_input.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-' || c == '_' || c == ':' || c == '.')
            return c;
        return '\0';
    };

    server_name_input.setScale(3);
    server_name_input.orientation = gfx::CENTER;
    server_name_input.setText("");
    server_name_input.active = true;
    server_name_input.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-' || c == '_' || c == ':' || c == '.')
            return c;
        return '\0';
    };

    server_name_input.y = - 16 - server_name_input.getHeight() / 2;
    server_ip_input.y = 16 + server_ip_input.getHeight() / 2;

    new_server_name.loadFromSurface(gfx::textToSurface("New server name"));
    new_server_name.setScale(3);
    new_server_name.y = server_name_input.y;
    new_server_name.x = -new_server_name.getWidth() / 2 + new_server_name.getWidth() / 2 + 16;//with commenting out this line the text will go to the center, choice will be made later
    new_server_name.orientation = gfx::CENTER;

    new_server_ip.loadFromSurface(gfx::textToSurface("New server ip"));
    new_server_ip.setScale(3);
    new_server_ip.y = server_ip_input.y;
    new_server_ip.x = -new_server_ip.getWidth() / 2 + new_server_ip.getWidth() / 2 + 16;//with commenting out this line the text will go to the center, choice will be made later
    new_server_ip.orientation = gfx::CENTER;
    
    server_name_input.def_color.a = TRANSPARENCY;
    server_ip_input.def_color.a = TRANSPARENCY;

    text_inputs = {&server_name_input, &server_ip_input};
}

bool ServerAdder::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered(getMouseX(), getMouseY())) {
        returnFromScene();
        return true;
    } else if((key == gfx::Key::MOUSE_LEFT && add_button.isHovered(getMouseX(), getMouseY())) || (key == gfx::Key::ENTER && can_add)) {
        server_ip = server_ip_input.getText();
        server_name = server_name_input.getText();
        returnFromScene();
        return true;
    }
    return false;
}

void ServerAdder::render() {
    menu_back->setBackWidth(server_ip_input.getWidth() + 100);
    menu_back->renderBack();
    if(can_add != !(server_ip_input.getText().empty() || server_name_input.getText().empty())) {
        if(can_add) // I know its ugly but if I do can_add = !can_add; for some reason clang-tidy thinks the variable not changing, and starts recommending weird optimizations.
            can_add = false;
        else
            can_add = true;
        add_button.loadFromSurface(gfx::textToSurface("Add server", {(unsigned char)(can_add ? WHITE.r : GREY.r), (unsigned char)(can_add ? WHITE.g : GREY.g), (unsigned char)(can_add ? WHITE.b : GREY.b)}));
        add_button.disabled = !can_add;
    }

    if(server_name_input.isHovered(getMouseX(), getMouseY()))
        new_server_name.setColor({GFX_DEFAULT_BUTTON_COLOR.r, GFX_DEFAULT_BUTTON_COLOR.g, GFX_DEFAULT_BUTTON_COLOR.b, TRANSPARENCY});
    else
        new_server_name.setColor(GFX_DEFAULT_HOVERED_BUTTON_COLOR);

    if(server_ip_input.isHovered(getMouseX(), getMouseY())) {
        new_server_ip.setColor({GFX_DEFAULT_BUTTON_COLOR.r, GFX_DEFAULT_BUTTON_COLOR.g, GFX_DEFAULT_BUTTON_COLOR.b, TRANSPARENCY});
    }
    else
        new_server_ip.setColor(GFX_DEFAULT_HOVERED_BUTTON_COLOR);


    add_button.render(getMouseX(), getMouseY(), getAbsoluteKeyState(gfx::Key::MOUSE_LEFT));
    back_button.render(getMouseX(), getMouseY(), getAbsoluteKeyState(gfx::Key::MOUSE_LEFT));
    add_server_title.render();
    server_ip_input.render(getMouseX(), getMouseY());
    server_name_input.render(getMouseX(), getMouseY());

    if(server_name_input.getText().empty())
        new_server_name.render();
    if(server_ip_input.getText().empty())
        new_server_ip.render();
}













