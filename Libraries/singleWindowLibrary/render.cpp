//
//  render.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"

void swl::setDrawColor(Uint8 r, Uint8 g, Uint8 b, Uint8 a) {
    SDL_SetRenderDrawColor(swl_private::renderer, r, g, b, a);
}

void swl::render(SDL_Texture* texture, SDL_Rect destination_rectangle, SDL_Rect source_rectangle, bool flipped) {
    SDL_RenderCopyEx(swl_private::renderer, texture, &source_rectangle, &destination_rectangle, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void swl::render(SDL_Texture* texture, SDL_Rect destination_rectangle, bool flipped) {
    SDL_RenderCopyEx(swl_private::renderer, texture, nullptr, &destination_rectangle, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}
void swl::render(SDL_Texture* texture, bool flipped) {
    SDL_RenderCopyEx(swl_private::renderer, texture, nullptr, nullptr, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void swl::render(SDL_Rect &rect, bool fill) {
    if(fill)
        SDL_RenderFillRect(swl_private::renderer, &rect);
    else
        SDL_RenderDrawRect(swl_private::renderer, &rect);
}
