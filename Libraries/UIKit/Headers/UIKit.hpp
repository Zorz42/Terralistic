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
    button(ogl::objectType type=ogl::centered);
    
    void setHoverColor(Uint8 r, Uint8 g, Uint8 b);
    void setTextColor(Uint8 r, Uint8 g, Uint8 b);
    using ogl::rect::setColor;
    
    void setText(std::string text_);
    void render();
    bool hovered();
    void setScale(Uint8 scale);
    bool isPressed(SDL_Event& event);
    void setMargin(unsigned short margin_);
    
    void setCenteredX(bool input);
    void setCenteredY(bool input);
    using ogl::rect::setX;
    using ogl::rect::setY;
    
    using ogl::rect::getWidth;
    using ogl::rect::getHeight;
    
protected:
    Uint8 hover_r, hover_g, hover_b;
    Uint8 text_r, text_g, text_b;
    ogl::texture text;
    unsigned short margin;
};
}

#endif /* UIKit_h */
