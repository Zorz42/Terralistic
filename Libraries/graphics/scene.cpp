#include "graphics-internal.hpp"

bool key_states[(int)gfx::Key::UNKNOWN];

void gfx::Scene::registerAModule(SceneModule* module) {
    modules.push_back(module);
    module->init();
}

const std::vector<gfx::SceneModule*>& gfx::Scene::getModules() {
    return modules;
}

short gfx::Scene::getMouseX() {
    return mouse_x;
}

short gfx::Scene::getMouseY() {
    return mouse_y;
}

void gfx::Scene::onKeyDownCallback(Key key_) {
    if(!key_states[(int)key_]) {
        key_states[(int)key_] = true;
        for(int i = (int)modules.size() - 1; i >= 0; i--)
            if(modules[i]->onKeyDown(key_))
                break;
    }
}

bool gfx::SceneModule::getKeyState(Key key_) const {
    return enable_key_states && key_states[(int)key_];
}

gfx::Key translateMouseKey(sf::Mouse::Button sfml_button) {
    switch(sfml_button) {
        case sf::Mouse::Left: return gfx::Key::MOUSE_LEFT;
        case sf::Mouse::Middle: return gfx::Key::MOUSE_MIDDLE;
        case sf::Mouse::Right: return gfx::Key::MOUSE_RIGHT;
        default: return gfx::Key::UNKNOWN;
    }
}

void gfx::Scene::switchToScene(Scene& scene) {
    for(SceneModule* module : modules)
        module->enable_key_states = false;
    scene.run();
    for(SceneModule* module : modules)
        module->enable_key_states = true;
}

void gfx::Scene::returnFromScene() {
    running = false;
}

gfx::Key translateKeyboardKey(sf::Keyboard::Key sfml_button) {
    switch(sfml_button) {
        case sf::Keyboard::Key::A: return gfx::Key::A;
        case sf::Keyboard::Key::B: return gfx::Key::B;
        case sf::Keyboard::Key::C: return gfx::Key::C;
        case sf::Keyboard::Key::D: return gfx::Key::D;
        case sf::Keyboard::Key::E: return gfx::Key::E;
        case sf::Keyboard::Key::F: return gfx::Key::F;
        case sf::Keyboard::Key::G: return gfx::Key::G;
        case sf::Keyboard::Key::H: return gfx::Key::H;
        case sf::Keyboard::Key::I: return gfx::Key::I;
        case sf::Keyboard::Key::J: return gfx::Key::J;
        case sf::Keyboard::Key::K: return gfx::Key::K;
        case sf::Keyboard::Key::L: return gfx::Key::L;
        case sf::Keyboard::Key::M: return gfx::Key::M;
        case sf::Keyboard::Key::N: return gfx::Key::N;
        case sf::Keyboard::Key::O: return gfx::Key::O;
        case sf::Keyboard::Key::P: return gfx::Key::P;
        case sf::Keyboard::Key::Q: return gfx::Key::Q;
        case sf::Keyboard::Key::R: return gfx::Key::R;
        case sf::Keyboard::Key::S: return gfx::Key::S;
        case sf::Keyboard::Key::T: return gfx::Key::T;
        case sf::Keyboard::Key::U: return gfx::Key::U;
        case sf::Keyboard::Key::V: return gfx::Key::V;
        case sf::Keyboard::Key::W: return gfx::Key::W;
        case sf::Keyboard::Key::X: return gfx::Key::X;
        case sf::Keyboard::Key::Y: return gfx::Key::Y;
        case sf::Keyboard::Key::Z: return gfx::Key::Z;
        case sf::Keyboard::Key::Num0: return gfx::Key::NUM0;
        case sf::Keyboard::Key::Num1: return gfx::Key::NUM1;
        case sf::Keyboard::Key::Num2: return gfx::Key::NUM2;
        case sf::Keyboard::Key::Num3: return gfx::Key::NUM3;
        case sf::Keyboard::Key::Num4: return gfx::Key::NUM4;
        case sf::Keyboard::Key::Num5: return gfx::Key::NUM5;
        case sf::Keyboard::Key::Num6: return gfx::Key::NUM6;
        case sf::Keyboard::Key::Num7: return gfx::Key::NUM7;
        case sf::Keyboard::Key::Num8: return gfx::Key::NUM8;
        case sf::Keyboard::Key::Num9: return gfx::Key::NUM9;
        case sf::Keyboard::Key::Space: return gfx::Key::SPACE;
        case sf::Keyboard::Key::Escape: return gfx::Key::ESCAPE;
        case sf::Keyboard::Key::Enter: return gfx::Key::ENTER;
        case sf::Keyboard::Key::LShift:
        case sf::Keyboard::Key::RShift: return gfx::Key::SHIFT;
        case sf::Keyboard::Key::Backspace: return gfx::Key::BACKSPACE;
        case sf::Keyboard::Key::LControl: case sf::Keyboard::Key::RControl: return gfx::Key::CTRL;
        default: return gfx::Key::UNKNOWN;
    }
}

void gfx::Scene::onEvent(sf::Event event) {
    if (event.type == sf::Event::Resized)
        setWindowSize(event.size.width / global_scale, event.size.height / global_scale);
    
    else if (event.type == sf::Event::MouseButtonPressed) {
        gfx::Key key = translateMouseKey(event.mouseButton.button);
        bool clicked_text_box = false;
        if (key == Key::MOUSE_LEFT) {
            for (SceneModule* module : modules)
                for (TextInput* i : module->text_inputs) {
                    i->active = i->isHovered(getMouseX(), getMouseY());
                    if (i->active)
                        clicked_text_box = true;
                }
        }
        if (key != Key::UNKNOWN && !clicked_text_box)
            onKeyDownCallback(key);
    }
    
    else if (event.type == sf::Event::MouseButtonReleased) {
        gfx::Key key = translateMouseKey(event.mouseButton.button);
        if (key != Key::UNKNOWN)
            key_states[(int)key] = false;
    }
    
    else if (event.type == sf::Event::KeyPressed) {
        gfx::Key key = translateKeyboardKey(event.key.code);
        if (key == Key::BACKSPACE) {
            for (SceneModule* module : modules)
                for (TextInput* i : module->text_inputs)
                    if (i->active && !i->getText().empty()) {
                        std::string str = i->getText();
                        str.pop_back();
                        i->setText(str);
                    }
        }
        bool is_textbox_active = false;
        for(SceneModule* module : modules)
            for(TextInput* i : module->text_inputs)
                if(i->active)
                    is_textbox_active = true;
        
        if(key != Key::UNKNOWN && (!is_textbox_active || key == Key::ENTER))
            onKeyDownCallback(key);
        
        if(is_textbox_active && key == Key::ESCAPE) {
            for(SceneModule* module : modules)
                for(TextInput* i : module->text_inputs)
                    i->active = false;
        }
    }
    
    else if (event.type == sf::Event::KeyReleased) {
        gfx::Key key = translateKeyboardKey(event.key.code);
        if (key != Key::UNKNOWN)
            key_states[(int)key] = false;
    }
    
    else if (event.type == sf::Event::TextEntered) {
        char c = event.text.unicode;
        if(c == '\b')
            return;
    
        for (SceneModule* module : modules)
            for (TextInput* i : module->text_inputs)
                if (i->active) {
                    char result = c;
                    if (!i->ignore_next_input) {
                        if (i->textProcessing)
                            result = i->textProcessing(result, (int)i->getText().size());
                        if (result)
                            i->setText(i->getText() + result);
                    }
                    i->ignore_next_input = false;
                }
    }
    
    else if (event.type == sf::Event::MouseWheelScrolled)
        onMouseScroll(event.mouseWheelScroll.delta);
    
    else if (event.type == sf::Event::Closed)
        window->close();
}

void gfx::Scene::run() {
    modules.push_back(this);
    init();
    
    while(running && window->isOpen()) {
        unsigned int start = getTicks();
        
        mouse_x = sf::Mouse::getPosition(*window).x / global_scale;
        mouse_y = sf::Mouse::getPosition(*window).y / global_scale;
        for(SceneModule* module : modules) {
            module->mouse_x = mouse_x;
            module->mouse_y = mouse_y;
        }
        
        sf::Event event;
        while(window->pollEvent(event))
            onEvent(event);
        
        for(SceneModule* module : modules)
            module->update();
        
        for(SceneModule* module : modules)
            module->render();
        
        window_texture.display();
        window->draw(sf::Sprite(window_texture.getTexture()));
        window->display();
        
        frame_length = getTicks() - start;
    }
    
    for(SceneModule* module : modules)
        module->stop();
}
