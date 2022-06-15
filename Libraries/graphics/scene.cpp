#include "graphics-internal.hpp"

void gfx::Scene::initialize() {
    if(!initialized) {
        for(int i = 0; i < modules.size(); i++)
            modules[i]->init();
        modules.insert(modules.begin(), this);
        init();
        initialized = true;
    }
}

void gfx::Scene::registerAModule(SceneModule* module) {
    modules.push_back(module);
}

const std::vector<gfx::SceneModule*>& gfx::Scene::getModules() {
    return modules;
}

int gfx::SceneModule::getMouseX() {
    return mouse_x;
}

int gfx::SceneModule::getMouseY() {
    return mouse_y;
}

void gfx::Scene::onKeyDownCallback(Key key_) {
    if(!key_states[(int)key_]) {
        key_states[(int)key_] = true;
        for(int i = (int)modules.size() - 1; i >= 0; i--)
            if(modules[i]->enabled && modules[i]->onKeyDown(key_))
                break;
    }
}

void gfx::Scene::onKeyUpCallback(Key key_) {
    if(key_states[(int)key_]) {
        key_states[(int)key_] = false;
        for(int i = (int)modules.size() - 1; i >= 0; i--)
            if(modules[i]->enabled && modules[i]->onKeyUp(key_))
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
    for(int i = 0; i < modules.size(); i++)
        modules[i]->enable_key_states = false;
    scene.run();
    for(int i = 0; i < modules.size(); i++)
        modules[i]->enable_key_states = true;
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
        case sf::Keyboard::Key::Up: return gfx::Key::ARROW_UP;
        case sf::Keyboard::Key::Down: return gfx::Key::ARROW_DOWN;
        case sf::Keyboard::Key::Right: return gfx::Key::ARROW_RIGHT;
        case sf::Keyboard::Key::Left: return gfx::Key::ARROW_LEFT;
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
                if(module->enabled)
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
            onKeyUpCallback(key);
    }
    
    else if (event.type == sf::Event::KeyPressed) {
        gfx::Key key = translateKeyboardKey(event.key.code);
        if (key == Key::BACKSPACE) {
            for (SceneModule* module : modules)
                for (TextInput* i : module->text_inputs)
                    if (i->active && !i->getText().empty()) {
                        std::string str = i->getText();
                        if(i->getCursorBegin() != i->getCursorEnd()) {
                            i->eraseSelected();
                        }else{
                            if(i->getCursorBegin() != 0) {
                                str.erase(i->getCursorBegin() - 1, 1);
                                i->setCursor(i->getCursorBegin() - 1);
                                i->setText(str);
                            }
                        }
                    }
        }

        if(key == Key::A && sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LControl)){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active)
                            text_input->setCursor(0, text_input->getText().size());
        }

        if(key == Key::C && sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LControl)){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active)
                            text_input->clipboard.setString(text_input->getText().substr(text_input->getCursorBegin(), text_input->getCursorEnd() - text_input->getCursorBegin()));
        }

        if(key == Key::X && sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LControl)){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active) {
                            text_input->clipboard.setString(text_input->getText().substr(text_input->getCursorBegin(), text_input->getCursorEnd() - text_input->getCursorBegin()));
                            text_input->eraseSelected();
                        }
        }

        if(key == Key::V && sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LControl)){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active) {
                            text_input->eraseSelected();
                            std::string temp_str = text_input->getText();
                            temp_str.insert(text_input->getCursorBegin(), text_input->clipboard.getString());
                            text_input->setText(temp_str);
                            text_input->setCursor(text_input->getCursorBegin() + text_input->clipboard.getString().getSize());
                        }
        }

        if(key == Key::ARROW_LEFT){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active)
                            if(sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LShift) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::RShift)){
                                if(text_input->getCursorBegin() == text_input->getCursorEnd()){
                                    text_input->setCursor(std::max(0, text_input->getCursorBegin() - 1), text_input->getCursorBegin());
                                    text_input->setCursorEndActive(false);
                                }
                                else{
                                    if(text_input->getCursorEndActive())
                                        text_input->setCursor(text_input->getCursorBegin(), text_input->getCursorEnd() - 1);
                                    else
                                        text_input->setCursor(std::max(0, text_input->getCursorBegin() - 1), text_input->getCursorEnd());
                                }
                            }else {
                                if(text_input->getCursorBegin() == text_input->getCursorEnd())
                                    text_input->setCursor(std::max(0, text_input->getCursorBegin() - 1));
                                else
                                    text_input->setCursor(text_input->getCursorBegin());
                            }
        }

        if(key == Key::ARROW_RIGHT){
            for(auto & module : modules)
                if(module->enabled)
                    for(auto & text_input : module->text_inputs)
                        if(text_input->active)
                            if(sf::Keyboard::isKeyPressed(sf::Keyboard::Key::LShift) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::RShift)){
                                if(text_input->getCursorBegin() == text_input->getCursorEnd()){
                                    text_input->setCursor(text_input->getCursorBegin(), std::min((int) text_input->getText().size(), text_input->getCursorBegin() + 1));
                                    text_input->setCursorEndActive(true);
                                }
                                else{
                                    if(text_input->getCursorEndActive())
                                        text_input->setCursor(text_input->getCursorBegin(), std::min((int) text_input->getText().size(), text_input->getCursorEnd() + 1));
                                    else
                                        text_input->setCursor(text_input->getCursorBegin() + 1, text_input->getCursorEnd());
                                }
                            }else {
                                if (text_input->getCursorBegin() == text_input->getCursorEnd())
                                    text_input->setCursor(std::min((int) text_input->getText().size(), text_input->getCursorBegin() + 1));
                                else
                                    text_input->setCursor(text_input->getCursorEnd());
                            }
        }


        bool is_textbox_active = false;
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active) {
                        is_textbox_active = true;
                        std::vector<Key> input_passthrough_keys = text_input->getPassthroughKeys();
                        if(std::count(input_passthrough_keys.begin(), input_passthrough_keys.end(), key))
                            module->onKeyDown(key);
                    }
        
        if(key != Key::UNKNOWN && (!is_textbox_active || key == Key::ENTER))
            onKeyDownCallback(key);
        
        if(is_textbox_active && key == Key::ESCAPE) {
            for(int i = 0; i < modules.size(); i++)
                if(modules[i]->enabled)
                    for(int i2 = 0; i2 < modules[i]->text_inputs.size(); i2++)
                        modules[i]->text_inputs[i2]->active = false;
        }
    }
    
    else if (event.type == sf::Event::KeyReleased) {
        gfx::Key key = translateKeyboardKey(event.key.code);
        if (key != Key::UNKNOWN)
            onKeyUpCallback(key);
    }
    
    else if (event.type == sf::Event::TextEntered) {
        char c = event.text.unicode;
        if(c == '\b')
            return;
    
        for (SceneModule* module : modules)
            if(module->enabled)
                for (TextInput* i : module->text_inputs)
                    if (i->active) {
                        char result = c;
                        if (!i->ignore_next_input) {
                            if (i->textProcessing)
                                result = i->textProcessing(result, (int)i->getText().size());
                            if (result) {
                                i->eraseSelected();
                                std::string new_text = i->getText();
                                new_text.insert(new_text.begin() + i->getCursorBegin(), result);
                                i->setCursor(i->getCursorBegin() + 1, i->getCursorBegin() + 1);
                                i->setText(new_text);
                            }
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
    initialize();
    
    while(running) {
        Timer frame_timer;
        
        mouse_x = sf::Mouse::getPosition(*window).x / global_scale;
        mouse_y = sf::Mouse::getPosition(*window).y / global_scale;
        for(int i = 0; i < modules.size(); i++)
            if(modules[i]->enabled) {
                modules[i]->mouse_x = mouse_x;
                modules[i]->mouse_y = mouse_y;
            }
        
        sf::Event event;
        
        while(window->pollEvent(event))
            onEvent(event);
        
        cycleModules();
        
        for(int i = 0; i < global_update_functions.size(); i++)
            global_update_functions[i]->update();
        
        window_texture.display();
        sf::Sprite window_sprite(window_texture.getTexture());
        window_sprite.setPosition(0, 0);
        window->draw(window_sprite);
        window->display();
        
        frame_length = frame_timer.getTimeElapsed();
        
        if(!window->isOpen())
            running = false;
    }
    
    for(int i = 0; i < modules.size(); i++)
        modules[i]->stop();
}

void gfx::Scene::cycleModules() {
    for(int i = 0; i < modules.size(); i++)
        modules[i]->update(frame_length);
    
    for(int i = 0; i < modules.size(); i++)
        if(modules[i]->enabled)
            modules[i]->render();
}

void gfx::Scene::returnFromScene() {
    running = false;
}

bool gfx::Scene::isInitialized() {
    return initialized;
}

bool gfx::Scene::isRunning() {
    return running;
}
