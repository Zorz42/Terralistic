//
//  objectedGraphicsLibrary.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#ifndef objectedGraphicsLibrary_hpp
#define objectedGraphicsLibrary_hpp

#include <SDL2/SDL.h>
#include "singleWindowLibrary.hpp"

namespace ogl_private {

class centeredObject {
public:
    void setX(short x_) { x = x_; }
    void setY(short y_) { y = y_; }
    
    inline virtual unsigned short getWidth() { return width; };
    inline virtual unsigned short getHeight() { return height; };
    
    void setOrientation(Uint8 objectType);
    
    swl::rect getRect();
    
protected:
    short x = 0, y = 0;
    unsigned short width = 0, height = 0;
    
    short getX();
    short getY();
    
    Uint8 orientation_x{}, orientation_y{};
};

}

namespace ogl {

enum objectType {top_left, top, top_right [[maybe_unused]], left, center, right, bottom_left [[maybe_unused]], bottom, bottom_right};

// texture that gets centered in window
class texture : public ogl_private::centeredObject {
public:
    explicit texture(objectType type=center);
    ~texture();
    
    void setTexture(SDL_Texture* input_texture, unsigned short width_, unsigned short height_);
    void setTexture(SDL_Texture* input_texture);
    
    void render();
    
    void loadFromText(const std::string& text, SDL_Color text_color);
    void loadFromFile(const std::string& path);
    
    inline unsigned short getWidth() override { return width * scale; };
    inline unsigned short getHeight() override { return height * scale; };
    
    bool flipped{false};
    
    Uint8 scale = 1;
    bool free_texture = true;
    
protected:
    SDL_Texture* texture_ = nullptr;
    void freeTexture();
};

// rect that gets centered in window
class rect : public ogl_private::centeredObject {
public:
    explicit rect(objectType type=center);
    
    inline void setWidth(unsigned short width_) { width = width_; }
    inline void setHeight(unsigned short height_) { height = height_; }
    
    void setColor(Uint8 r_, Uint8 g_, Uint8 b_);

    virtual void render();

    bool touchesPoint(short x, short y);
    bool fill = true;
    
    using ogl_private::centeredObject::getX;
    using ogl_private::centeredObject::getY;
    
protected:
    Uint8 r{}, g{}, b{};
};

}


#endif /* objectedGraphicsLibrary_hpp */
