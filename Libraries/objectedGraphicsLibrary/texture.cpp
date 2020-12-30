//
//  centeredTexture.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include <utility>

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

ogl::texture::texture(ogl::objectType type) {
    setOrientation(type);
}

void ogl::texture::setTexture(SDL_Texture* input_texture, unsigned short width_, unsigned short height_) {
    freeTexture();
    texture_ = input_texture;
    width = width_;
    height = height_;
}

void ogl::texture::setTexture(SDL_Texture* input_texture) {
    freeTexture();
    texture_ = input_texture;
}

void ogl::texture::render() {
    SDL_Rect render_rect = getRect();
    swl::render(texture_, render_rect, flipped);
}

void ogl::texture::freeTexture() {
    if(texture_ && free_texture)
        SDL_DestroyTexture(texture_);
}

ogl::texture::~texture() {
    freeTexture();
}

void ogl::texture::loadFromText(const std::string& text, SDL_Color text_color) {
    unsigned short temp_width, temp_height;
    setTexture(swl::loadTextureFromText(text, text_color, &temp_width, &temp_height));
    width = temp_width;
    height = temp_height;
}

void ogl::texture::loadFromFile(std::string path) {
    unsigned short temp_width, temp_height;
    setTexture(swl::loadTextureFromFile(std::move(path), &temp_width, &temp_height));
    width = temp_width;
    height = temp_height;
}
