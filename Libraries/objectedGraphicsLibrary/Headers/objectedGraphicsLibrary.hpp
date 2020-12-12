//
//  objectedGraphicsLibrary.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#ifndef objectedGraphicsLibrary_hpp
#define objectedGraphicsLibrary_hpp

#include <iostream>
#include <SDL2/SDL.h>

namespace __ogl_private {

class centeredObject {
public:
    void setX(short x_);
    void setY(short y_);
    
    inline virtual short getWidth() { return width; };
    inline virtual short getHeight() { return height; };
    
    void setOrientation(Uint8 objectType);
    
    SDL_Rect getRect();
    
protected:
    short x = 0, y = 0;
    unsigned short width = 0, height = 0;
    
    short getX();
    short getY();
    
    Uint8 orientation_x, orientation_y;
};

}

namespace ogl {

enum objectType {top_left, top, top_right, left, center, right, bottom_left, bottom, bottom_right};

// texture that gets centered in window
class texture : public __ogl_private::centeredObject {
public:
    texture(objectType type=center);
    ~texture();
    
    void setTexture(SDL_Texture* input_texture, int width_, int height_);
    
    void render();
    
    void loadFromText(std::string text, SDL_Color text_color);
    void loadFromFile(std::string path);
    
    inline short getWidth() { return width * scale; };
    inline short getHeight() { return height * scale; };
    
    bool flipped{false};
    
    Uint8 scale = 1;
    bool free_texture = true;
    
protected:
    SDL_Texture* texture_ = nullptr;
    void freeTexture();
};

// rect that gets centered in window
class rect : public __ogl_private::centeredObject {
public:
    rect(objectType type=center);
    
    inline void setWidth(short width_) { width = width_; }
    inline void setHeight(short height_) { height = height_; }
    
    void setColor(Uint8 r_, Uint8 g_, Uint8 b_);
    void render();

    bool touchesPoint(int x, int y);
    bool fill = true;
    
protected:
    Uint8 r, g, b;
};

}


#endif /* objectedGraphicsLibrary_hpp */
