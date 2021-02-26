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

// render misc things

void swl::render(SDL_Texture* texture, swl::rect destination_rectangle, swl::rect source_rectangle, bool flipped) {
    SDL_Rect render_rect_source = {source_rectangle.x, source_rectangle.y, source_rectangle.w, source_rectangle.h};
    SDL_Rect render_rect_dest = {destination_rectangle.x, destination_rectangle.y, destination_rectangle.w, destination_rectangle.h};
    SDL_RenderCopyEx(swl_private::renderer, texture, &render_rect_source, &render_rect_dest, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void swl::render(SDL_Texture* texture, swl::rect destination_rectangle, bool flipped) {
    SDL_Rect render_rect = {destination_rectangle.x, destination_rectangle.y, destination_rectangle.w, destination_rectangle.h};
    SDL_RenderCopyEx(swl_private::renderer, texture, nullptr, &render_rect, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}
void swl::render(SDL_Texture* texture, bool flipped) {
    SDL_RenderCopyEx(swl_private::renderer, texture, nullptr, nullptr, 0, nullptr, flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void swl::render(swl::rect &rect, bool fill) {
    SDL_Rect render_rect = {rect.x, rect.y, rect.w, rect.h};
    if(fill)
        SDL_RenderFillRect(swl_private::renderer, &render_rect);
    else
        SDL_RenderDrawRect(swl_private::renderer, &render_rect);
}
