//
//  scenes.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#include "graphics-internal.hpp"

void gfx::switchScene(scene* x) {
    x->init();
    scene_stack.push(x);
}

void gfx::returnFromScene() {
    scene_stack.top()->stop();
    delete scene_stack.top();
    scene_stack.pop();
}

void gfx::runScenes() {
    bool quit = false;
    SDL_Event event;
    
    scene* used_scene = nullptr;
    
    while(scene_stack.size()) {
        used_scene = scene_stack.top();
        
        while(SDL_PollEvent(&event)) {
            if(event.type == SDL_QUIT)
                quit = true;
            else if(event.type == SDL_MOUSEMOTION)
                SDL_GetMouseState((int*)&mouse_x, (int*)&mouse_y);
            else if(event.type == SDL_WINDOWEVENT && event.window.event == SDL_WINDOWEVENT_RESIZED) {
                window_width = (unsigned short)event.window.data1;
                window_height = (unsigned short)event.window.data2;
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
