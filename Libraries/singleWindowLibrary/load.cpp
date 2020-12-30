//
//  load.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include <SDL2_image/SDL_image.h>
#include "singleWindowLibrary.hpp"

SDL_Texture* swl::loadTextureFromFile(std::string path, unsigned short* w, unsigned short* h) {
    path = swl_private::resourcePath + path;
    
    SDL_Surface *loaded_surface = IMG_Load(path.c_str());
    if(!loaded_surface)
        swl::popupError(path + " could not be loaded!");
    
    SDL_SetColorKey(loaded_surface, SDL_TRUE, SDL_MapRGB(loaded_surface->format, 0, 255, 0));
    
    SDL_Texture* result = SDL_CreateTextureFromSurface(swl_private::renderer, loaded_surface);
    if(!result)
        swl::popupError(path + " could not be transformed into a texture!");
    
    if(w)
        *w = (unsigned short)loaded_surface->w;
    if(h)
        *h = (unsigned short)loaded_surface->h;
    
    SDL_FreeSurface(loaded_surface);
    
    return result;
}

void swl::loadFont(std::string path, unsigned char size) {
    path = swl_private::resourcePath + path;
    swl_private::font = TTF_OpenFont(path.c_str(), size);
    if(!swl_private::font)
        swl::popupError(path + " could not be loaded!");
}

SDL_Texture* swl::loadTextureFromText(const std::string& text, SDL_Color text_color, unsigned short* w, unsigned short* h) {
    SDL_Surface *rendered_surface = TTF_RenderText_Solid(swl_private::font, text.c_str(), text_color);
    if(!rendered_surface)
        swl::popupError("Surface with text \"" + text + "\" could not be rendered!");

    SDL_Texture* result = SDL_CreateTextureFromSurface(swl_private::renderer, rendered_surface);
    if(!result)
        swl::popupError("Surface with text \"" + text + "\" could not be transformed into a texture!");
    
    if(w)
        *w = (unsigned short)rendered_surface->w;
    if(h)
        *h = (unsigned short)rendered_surface->h;

    SDL_FreeSurface(rendered_surface);

    return result;
}
