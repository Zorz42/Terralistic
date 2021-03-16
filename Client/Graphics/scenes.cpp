//
//  scenes.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#include "graphics-internal.hpp"

#include <iostream>

gfx::scene* used_scene = nullptr;

void gfx::switchScene(scene* x) {
    x->init();
    scene_stack.push(x);
    used_scene = x;
}

void gfx::returnFromScene() {
    scene_stack.pop();
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
        case SDLK_SPACE: return gfx::KEY_SPACE;
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
        default: return gfx::KEY_UNKNOWN;
    }
}

void gfx::runScenes() {
    bool quit = false;
    SDL_Event event;
    
    while(scene_stack.size()) {
        if(used_scene != scene_stack.top()) {
            delete used_scene;
            used_scene = scene_stack.top();
        }
        
        while(SDL_PollEvent(&event)) {
            if(event.type == SDL_QUIT)
                quit = true;
            else if(event.type == SDL_MOUSEMOTION)
                SDL_GetMouseState((int*)&mouse_x, (int*)&mouse_y);
            else if(event.type == SDL_WINDOWEVENT && event.window.event == SDL_WINDOWEVENT_RESIZED) {
                window_width = (unsigned short)event.window.data1;
                window_height = (unsigned short)event.window.data2;
            } else if(event.type == SDL_MOUSEBUTTONDOWN) {
                gfx::key key = translateMouseKey(event.button.button);
                if(key != KEY_UNKNOWN)
                    used_scene->onKeyDown(key);
            } else if(event.type == SDL_MOUSEBUTTONUP) {
                gfx::key key = translateMouseKey(event.button.button);
                if(key != KEY_UNKNOWN)
                    used_scene->onKeyUp(key);
            } else if(event.type == SDL_KEYDOWN) {
                gfx::key key = translateMouseKey(event.key.keysym.sym);
                if(key != KEY_UNKNOWN)
                    used_scene->onKeyDown(key);
            } else if(event.type == SDL_KEYUP) {
                gfx::key key = translateMouseKey(event.key.keysym.sym);
                if(key != KEY_UNKNOWN)
                    used_scene->onKeyUp(key);
            }
        }
        
        used_scene->update();
        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
        SDL_RenderClear(renderer);
        
        used_scene->render();
        
        SDL_RenderPresent(renderer);
        if(quit)
            while(scene_stack.size())
                returnFromScene();
    }
}
