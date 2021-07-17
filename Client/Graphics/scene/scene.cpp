#include "graphics-internal.hpp"

static bool running_scene = true, disable_events_gl;

void gfx::Scene::onKeyDownCallback(key key_) {
    if(!disable_events_gl || disable_events)
        onKeyDown(key_);
    for(GraphicalModule* module : modules)
        if(!disable_events_gl || module->disable_events)
            module->onKeyDown(key_);
}

void gfx::Scene::onKeyUpCallback(key key_) {
    if (!disable_events_gl || disable_events)
        onKeyUp(key_);
    for (GraphicalModule* module : modules)
        if (!disable_events_gl || module->disable_events)
            module->onKeyUp(key_);
 }

gfx::key translateMouseKey(sf::Mouse::Button sfml_button) {
    switch(sfml_button) {
        case sf::Mouse::Left: return gfx::KEY_MOUSE_LEFT;
        case sf::Mouse::Middle: return gfx::KEY_MOUSE_MIDDLE;
        case sf::Mouse::Right: return gfx::KEY_MOUSE_RIGHT;
        default: return gfx::KEY_UNKNOWN;
    }
}

gfx::key translateKeyboardKey(sf::Keyboard::Key sfml_button) {
    switch(sfml_button) {
        case sf::Keyboard::Key::A: return gfx::KEY_A;
        case sf::Keyboard::Key::B: return gfx::KEY_B;
        case sf::Keyboard::Key::C: return gfx::KEY_C;
        case sf::Keyboard::Key::D: return gfx::KEY_D;
        case sf::Keyboard::Key::E: return gfx::KEY_E;
        case sf::Keyboard::Key::F: return gfx::KEY_F;
        case sf::Keyboard::Key::G: return gfx::KEY_G;
        case sf::Keyboard::Key::H: return gfx::KEY_H;
        case sf::Keyboard::Key::I: return gfx::KEY_I;
        case sf::Keyboard::Key::J: return gfx::KEY_J;
        case sf::Keyboard::Key::K: return gfx::KEY_K;
        case sf::Keyboard::Key::L: return gfx::KEY_L;
        case sf::Keyboard::Key::M: return gfx::KEY_M;
        case sf::Keyboard::Key::N: return gfx::KEY_N;
        case sf::Keyboard::Key::O: return gfx::KEY_O;
        case sf::Keyboard::Key::P: return gfx::KEY_P;
        case sf::Keyboard::Key::Q: return gfx::KEY_Q;
        case sf::Keyboard::Key::R: return gfx::KEY_R;
        case sf::Keyboard::Key::S: return gfx::KEY_S;
        case sf::Keyboard::Key::T: return gfx::KEY_T;
        case sf::Keyboard::Key::U: return gfx::KEY_U;
        case sf::Keyboard::Key::V: return gfx::KEY_V;
        case sf::Keyboard::Key::W: return gfx::KEY_W;
        case sf::Keyboard::Key::X: return gfx::KEY_X;
        case sf::Keyboard::Key::Y: return gfx::KEY_Y;
        case sf::Keyboard::Key::Z: return gfx::KEY_Z;
        case sf::Keyboard::Key::Num0: return gfx::KEY_0;
        case sf::Keyboard::Key::Num1: return gfx::KEY_1;
        case sf::Keyboard::Key::Num2: return gfx::KEY_2;
        case sf::Keyboard::Key::Num3: return gfx::KEY_3;
        case sf::Keyboard::Key::Num4: return gfx::KEY_4;
        case sf::Keyboard::Key::Num5: return gfx::KEY_5;
        case sf::Keyboard::Key::Num6: return gfx::KEY_6;
        case sf::Keyboard::Key::Num7: return gfx::KEY_7;
        case sf::Keyboard::Key::Num8: return gfx::KEY_8;
        case sf::Keyboard::Key::Num9: return gfx::KEY_9;
        case sf::Keyboard::Key::Space: return gfx::KEY_SPACE;
        case sf::Keyboard::Key::Escape: return gfx::KEY_ESCAPE;
        case sf::Keyboard::Key::Enter: return gfx::KEY_ENTER;
        case sf::Keyboard::Key::LShift:
        case sf::Keyboard::Key::RShift: return gfx::KEY_SHIFT;
        case sf::Keyboard::Key::Backspace: return gfx::KEY_BACKSPACE;
        default: return gfx::KEY_UNKNOWN;
    }
}

void gfx::returnFromScene() {
    running_scene = false;
}

void gfx::Scene::run() {
    static bool quit = false;
    //SDL_Event event;
    
    init();
    for(GraphicalModule* module : modules)
        module->init();
    
    //SDL_StartTextInput();
    while(running_scene && !quit) {
        unsigned int start = getTicks();
        
        disable_events_gl = disable_events;
        for(GraphicalModule* module : modules) {
            if(disable_events_gl)
                break;
            disable_events_gl = module->disable_events;
        }
        
        sf::Event event;
        while (sfml_window->pollEvent(event)) {
            if(event.type == sf::Event::MouseMoved) {
                mouse_x = event.mouseMove.x;
                mouse_y = event.mouseMove.y;
            } else if(event.type == sf::Event::Resized) {
                sf::FloatRect visibleArea(0, 0, (unsigned int)event.size.width, (unsigned int)event.size.height);
                sfml_window->setView(sf::View(visibleArea));
            } else if(event.type == sf::Event::MouseButtonPressed) {
                gfx::key key = translateMouseKey(event.mouseButton.button);
                bool clicked_text_box = false;
                if(key == KEY_MOUSE_LEFT) {
                    if(!disable_events_gl || disable_events)
                        for(TextInput* i : text_inputs) {
                            i->active = i->isHovered();
                            if(i->active)
                                clicked_text_box = true;
                        }
                    
                    for(GraphicalModule* module : modules)
                        if(!disable_events_gl || module->disable_events)
                            for(TextInput* i : module->text_inputs) {
                                i->active = i->isHovered();
                                if(i->active)
                                    clicked_text_box = true;
                            }
                }
                if(key != KEY_UNKNOWN && !clicked_text_box)
                    onKeyDownCallback(key);
            } else if(event.type == sf::Event::MouseButtonReleased) {
                gfx::key key = translateMouseKey(event.mouseButton.button);
                if(key != KEY_UNKNOWN)
                    onKeyUpCallback(key);
            } else if(event.type == sf::Event::KeyPressed) {
                gfx::key key = translateKeyboardKey(event.key.code);
                if(key == KEY_BACKSPACE) {
                    for(TextInput* i : text_inputs)
                        if(i->active && !i->getText().empty()) {
                            std::string str = i->getText();
                            str.pop_back();
                            i->setText(str);
                        }
                    for(GraphicalModule* module : modules)
                        for(TextInput* i : module->text_inputs)
                            if(i->active && !i->getText().empty()) {
                                std::string str = i->getText();
                                str.pop_back();
                                i->setText(str);
                            }
                }
                if(key != KEY_UNKNOWN)
                    onKeyDownCallback(key);
            } else if(event.type == sf::Event::KeyReleased) {
                gfx::key key = translateKeyboardKey(event.key.code);
                if(key != KEY_UNKNOWN)
                    onKeyUpCallback(key);
            } else if(event.type == sf::Event::TextEntered) {
                char c = event.text.unicode;
                
                for(TextInput* i : text_inputs)
                    if(i->active) {
                        char result = c;
                        if(!i->ignore_one_input) {
                            if(i->textProcessing)
                                result = i->textProcessing(result, (int)i->getText().size());
                            if(result)
                                i->setText(i->getText() + result);
                        }
                    i->ignore_one_input = false;
                    }
                for(GraphicalModule* module : modules)
                    for(TextInput* i : module->text_inputs)
                        if(i->active) {
                            char result = c;
                            if(!i->ignore_one_input) {
                                if(i->textProcessing)
                                    result = i->textProcessing(result, (int)i->getText().size());
                                if(result)
                                    i->setText(i->getText() + result);
                            }
                            i->ignore_one_input = false;
                        }
            } else if(event.type == sf::Event::MouseWheelScrolled) {
                onMouseScroll(event.mouseWheel.delta);
            } else if(event.type == sf::Event::Closed)
                quit = true;
        }
        
        update();
        for(GraphicalModule* module : modules)
            module->update();
        
        clearWindow();
        
        for(GraphicalModule* module : modules)
            module->render();
        render();
        
        updateWindow();
        
        unsigned int end = getTicks();
        frame_length = end - start;
        if(frame_length < 5) {
            sleep(5 - frame_length);
            frame_length = 5;
        }
    }
    
    running_scene = true;
    
    stop();
    for(GraphicalModule* module : modules) {
        module->stop();
        delete module;
    }
}
