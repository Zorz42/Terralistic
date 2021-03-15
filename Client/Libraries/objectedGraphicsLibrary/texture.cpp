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
    // free prev texture
    freeTexture();
    texture_ = input_texture;
    width = width_;
    height = height_;
}

void ogl::texture::setTexture(SDL_Texture* input_texture) {
    // same, but without dimensions
    freeTexture();
    texture_ = input_texture;
}

void ogl::texture::render() {
    swl::render(texture_, getRect(), flipped);
}

void ogl::texture::freeTexture() {
    // free texture can prevent destroying texture
    if(texture_ && free_texture)
        SDL_DestroyTexture(texture_);
}

ogl::texture::~texture() {
    freeTexture();
}

void ogl::texture::loadFromText(const std::string& text, SDL_Color text_color) {
    unsigned short temp_width, temp_height;
    setTexture(swl::loadTextureFromText(text, text_color, &temp_width, &temp_height), width, height);
    width = temp_width;
    height = temp_height;
}

void ogl::texture::loadFromFile(const std::string& path) {
    unsigned short temp_width, temp_height;
    setTexture(swl::loadTextureFromFile(path, &temp_width, &temp_height), width, height);
    width = temp_width;
    height = temp_height;
}
