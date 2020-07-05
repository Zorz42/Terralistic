//
//  render.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#include "singleWindowLibrary.hpp"

void swl::setDrawColor(Uint8 r, Uint8 g, Uint8 b, Uint8 a) {
    SDL_SetRenderDrawColor(__swl_private::renderer, r, g, b, a);
}

void swl::render(SDL_Texture* texture, SDL_Rect destination_rectangle, SDL_Rect source_rectangle) {
    SDL_RenderCopy(__swl_private::renderer, texture, &source_rectangle, &destination_rectangle);
}

void swl::render(SDL_Texture* texture, SDL_Rect destination_rectangle) {
    SDL_RenderCopy(__swl_private::renderer, texture, nullptr, &destination_rectangle);
}
void swl::render(SDL_Texture* texture) {
    SDL_RenderCopy(__swl_private::renderer, texture, nullptr, nullptr);
}

void swl::render(SDL_Rect &rect, bool fill) {
    if(fill)
        SDL_RenderFillRect(__swl_private::renderer, &rect);
    else
        SDL_RenderDrawRect(__swl_private::renderer, &rect);
}
