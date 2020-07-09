//
//  UIKit.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 07/07/2020.
//

#ifndef UIKit_h
#define UIKit_h

#include "objectedGraphicsLibrary.hpp"

namespace ui {
class button : protected ogl::rect {
public:
    button(ogl::objectType type=ogl::center);
    
    void setHoverColor(Uint8 r, Uint8 g, Uint8 b);
    void setTextColor(Uint8 r, Uint8 g, Uint8 b);
    using ogl::rect::setColor;
    
    void setText(std::string text_);
    void render();
    bool hovered();
    void setScale(Uint8 scale);
    bool isPressed(SDL_Event& event);
    void setMargin(unsigned short margin_);
    
    inline void setX(short x_) { x = x_; };
    inline void setY(short y_) { y = y_; };
    
    using ogl::rect::getWidth;
    using ogl::rect::getHeight;
    
protected:
    Uint8 hover_r, hover_g, hover_b;
    Uint8 text_r, text_g, text_b;
    ogl::texture text;
    unsigned short margin;
};

class loadingBar : protected ogl::rect {
public:
    loadingBar(unsigned short total_progress_, ogl::objectType type=ogl::center);
    
    using ogl::rect::setWidth;
    using ogl::rect::setHeight;
    using ogl::rect::render;
    using ogl::rect::setColor;
    void setBackColor(Uint8 r_, Uint8 g_, Uint8 b_);
    void render();
    
    void bind(void* progress_variable);
    
    using ogl::rect::setX;
    using ogl::rect::setY;
    
protected:
    unsigned short progress_width = 0, *current_progress = nullptr;
    unsigned short total_progress;
    Uint8 back_r, back_g, back_b;
};

}

#endif /* UIKit_h */
