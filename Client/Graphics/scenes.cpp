//
//  scenes.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#include "graphics-internal.hpp"

#include <iostream>

static bool running_scene = true, disable_events_gl;

void gfx::returnFromScene() {
    running_scene = false;
}

void gfx::scene::_onKeyDown(key key_) {
    if(!disable_events_gl || disable_events)
        onKeyDown(key_);
    for(sceneModule* module : modules)
        if(!disable_events_gl || module->disable_events)
            module->onKeyDown(key_);
}

void gfx::scene::_onKeyUp(key key_) {
    if(!disable_events_gl || disable_events)
        onKeyUp(key_);
    for(sceneModule* module : modules)
        if(!disable_events_gl || module->disable_events)
            module->onKeyUp(key_);
}

gfx::key translateMouseKey(int sdl_button) {
    switch(sdl_button) {
        case SDL_BUTTON_LEFT: return gfx::KEY_MOUSE_LEFT;
        case SDL_BUTTON_MIDDLE: return gfx::KEY_MOUSE_MIDDLE;
        case SDL_BUTTON_RIGHT: return gfx::KEY_MOUSE_RIGHT;
        case SDLK_a: return gfx::KEY_A;
        case SDLK_b: return gfx::KEY_B;
        case SDLK_c: return gfx::KEY_C;
        case SDLK_d: return gfx::KEY_D;
        case SDLK_e: return gfx::KEY_E;
        case SDLK_f: return gfx::KEY_F;
        case SDLK_g: return gfx::KEY_G;
        case SDLK_h: return gfx::KEY_H;
        case SDLK_i: return gfx::KEY_I;
        case SDLK_j: return gfx::KEY_J;
        case SDLK_k: return gfx::KEY_K;
        case SDLK_l: return gfx::KEY_L;
        case SDLK_m: return gfx::KEY_M;
        case SDLK_n: return gfx::KEY_N;
        case SDLK_o: return gfx::KEY_O;
        case SDLK_p: return gfx::KEY_P;
        case SDLK_q: return gfx::KEY_Q;
        case SDLK_r: return gfx::KEY_R;
        case SDLK_s: return gfx::KEY_S;
        case SDLK_t: return gfx::KEY_T;
        case SDLK_u: return gfx::KEY_U;
        case SDLK_v: return gfx::KEY_V;
        case SDLK_w: return gfx::KEY_W;
        case SDLK_x: return gfx::KEY_X;
        case SDLK_y: return gfx::KEY_Y;
        case SDLK_z: return gfx::KEY_Z;
        case SDLK_0: return gfx::KEY_0;
        case SDLK_1: return gfx::KEY_1;
        case SDLK_2: return gfx::KEY_2;
        case SDLK_3: return gfx::KEY_3;
        case SDLK_4: return gfx::KEY_4;
        case SDLK_5: return gfx::KEY_5;
        case SDLK_6: return gfx::KEY_6;
        case SDLK_7: return gfx::KEY_7;
        case SDLK_8: return gfx::KEY_8;
        case SDLK_9: return gfx::KEY_9;
        case SDLK_SPACE: return gfx::KEY_SPACE;
        case SDLK_ESCAPE: return gfx::KEY_ESCAPE;
        case SDLK_RETURN: return gfx::KEY_ENTER;
        default: return gfx::KEY_UNKNOWN;
    }
}

void gfx::runScene(scene* x) {
    static bool quit = false;
    SDL_Event event;
    
    x->init();
    for(sceneModule* module : x->modules)
        module->init();
    
    SDL_StartTextInput();
    while(running_scene && !quit) {
        Uint64 start = SDL_GetPerformanceCounter();
        
        disable_events_gl = x->disable_events;
        for(sceneModule* module : x->modules) {
            if(disable_events_gl)
                break;
            disable_events_gl = module->disable_events;
        }
            
        while(SDL_PollEvent(&event)) {
            if(event.type == SDL_MOUSEMOTION)
                SDL_GetMouseState((int*)&mouse_x, (int*)&mouse_y);
            else if(event.type == SDL_WINDOWEVENT && event.window.event == SDL_WINDOWEVENT_RESIZED) {
                window_width = (unsigned short)event.window.data1;
                window_height = (unsigned short)event.window.data2;
            } else if(event.type == SDL_MOUSEBUTTONDOWN) {
                gfx::key key = translateMouseKey(event.button.button);
                bool clicked_text_box = false;
                if(key == KEY_MOUSE_LEFT) {
                    if(!disable_events_gl || x->disable_events)
                        for(textInput* i : x->text_inputs) {
                            i->active = i->isHovered();
                            if(i->active)
                                clicked_text_box = true;
                        }
                    
                    for(sceneModule* module : x->modules)
                        if(!disable_events_gl || module->disable_events)
                            for(textInput* i : module->text_inputs) {
                                i->active = i->isHovered();
                                if(i->active)
                                    clicked_text_box = true;
                            }
                }
                if(key != KEY_UNKNOWN && !clicked_text_box)
                    x->_onKeyDown(key);
            } else if(event.type == SDL_MOUSEBUTTONUP) {
                gfx::key key = translateMouseKey(event.button.button);
                if(key != KEY_UNKNOWN)
                    x->_onKeyUp(key);
            } else if(event.type == SDL_KEYDOWN) {
                gfx::key key = translateMouseKey(event.key.keysym.sym);
                if(event.key.keysym.sym == SDLK_BACKSPACE) {
                    for(textInput* i : x->text_inputs)
                        if(i->active && !i->getText().empty()) {
                            std::string str = i->getText();
                            str.pop_back();
                            i->setText(str);
                        }
                    for(sceneModule* module : x->modules)
                        for(textInput* i : module->text_inputs)
                            if(i->active && !i->getText().empty()) {
                                std::string str = i->getText();
                                str.pop_back();
                                i->setText(str);
                            }
                }
                if(key != KEY_UNKNOWN)
                    x->_onKeyDown(key);
            } else if(event.type == SDL_KEYUP) {
                gfx::key key = translateMouseKey(event.key.keysym.sym);
                if(key != KEY_UNKNOWN)
                    x->_onKeyUp(key);
            } else if(event.type == SDL_TEXTINPUT) {
                std::string c = event.text.text;
                
                for(textInput* i : x->text_inputs)
                    if(i->active)
                        for(char result : c) {
                            if(!i->ignore_one_input) {
                                if(i->textProcessing)
                                    result = i->textProcessing(result, (int)i->getText().size());
                                if(result)
                                    i->setText(i->getText() + result);
                            }
                            i->ignore_one_input = false;
                        }
                for(sceneModule* module : x->modules)
                    for(textInput* i : module->text_inputs)
                        if(i->active)
                            for(char result : c) {
                                if(!i->ignore_one_input) {
                                    if(i->textProcessing)
                                        result = i->textProcessing(result, (int)i->getText().size());
                                    if(result)
                                        i->setText(i->getText() + result);
                                }
                                i->ignore_one_input = false;
                            }
                
            } else if(event.type == SDL_MOUSEWHEEL)
                x->onMouseScroll(event.wheel.y);
            else if(event.type == SDL_QUIT)
                quit = true;
        }
        
        x->update();
        for(sceneModule* module : x->modules)
            module->update();
        
        clearWindow();
        
        for(sceneModule* module : x->modules)
            module->render();
        x->render();
        
    
        updateWindow();
        
        Uint64 end = SDL_GetPerformanceCounter();
        frame_length = float(end - start) / (float)SDL_GetPerformanceFrequency() * 1000.0f;
        if(frame_length < 5) {
            SDL_Delay(5 - frame_length);
            frame_length = 5;
        }
    }
    
    running_scene = true;
    
    x->stop();
    for(sceneModule* module : x->modules) {
        module->stop();
        delete module;
    }
    
    delete x;
}
