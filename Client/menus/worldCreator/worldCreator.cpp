#include <algorithm>
#include "worldCreator.hpp"
#include "game.hpp"

void WorldCreator::init() {
    back_button.scale = 3;
    back_button.renderText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;
    
    new_world_title.renderText("New world name:");
    new_world_title.scale = 3;
    new_world_title.y = SPACING;
    new_world_title.orientation = gfx::TOP;
    
    create_button.scale = 3;
    create_button.renderText("Create world");
    create_button.y = -SPACING;
    create_button.orientation = gfx::BOTTOM;

    back_button.x = (-create_button.getWidth() - back_button.getWidth() + back_button.getWidth() - SPACING) / 2;
    create_button.x = (create_button.getWidth() + back_button.getWidth() - create_button.getWidth() + SPACING) / 2;

    world_name.scale = 3;
    world_name.orientation = gfx::CENTER;
    world_name.setText("");
    world_name.active = true;
    world_name.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '_')
            return c;
        if(c == ' ')
            return '-';
        return '\0';
    };
    
    world_name.def_color.a = TRANSPARENCY;
    
    text_inputs = {&world_name};
}

void WorldCreator::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::Key::MOUSE_LEFT && create_button.isHovered()) || (key == gfx::Key::ENTER && can_create)) {
        startPrivateWorld(world_name.getText(), menu_back);
        gfx::returnFromScene();
    }
}

void WorldCreator::render() {
    menu_back->setBackWidth(world_name.getWidth() + 100);
    menu_back->renderBack();
    if(can_create != (!world_name.getText().empty() && !std::count(worlds.begin(), worlds.end(), world_name.getText()))) {
        can_create = !can_create;
        create_button.renderText("Create world", {(unsigned char)(can_create ? WHITE.r : GREY.r), (unsigned char)(can_create ? WHITE.g : GREY.g), (unsigned char)(can_create ? WHITE.b : GREY.b)});
        create_button.disabled = !can_create;
    }
    create_button.render();
    back_button.render();
    new_world_title.render();
    world_name.render();
}
