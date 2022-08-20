#include "font.hpp"
#include "rectShape.hpp"

static gfx::Surface char_surfaces[256];

bool fontColEmpty(const gfx::Surface& surface, int x, int y) {
    for(int i = 0; i < 16; i++)
        if(surface.getPixel(x, y + i).a != 0)
            return false;
    
    return true;
}

void gfx::loadFont(const Surface& surface) {
    if(surface.getWidth() != 256 || surface.getHeight() != 256)
        throw Exception("Font surface must be 256x256");
    
    for(int y = 0; y < 16; y++)
        for(int x = 0; x < 16; x++) {
            RectShape rect(x * 16, y * 16, 16, 16);
            
            while(rect.w > 0 && fontColEmpty(surface, rect.x, rect.y)) {
                rect.x++;
                rect.w--;
            }
            
            while(rect.w > 0 && fontColEmpty(surface, rect.x + rect.w - 1, rect.y))
                rect.w--;
            
            if(y * 16 + x == ' ') {
                rect.x = x * 16;
                rect.w = 2;
            }
            
            Surface& char_surface = char_surfaces[y * 16 + x];
            char_surface.createEmpty(rect.w, rect.h);
            
            for(int y_ = 0; y_ < rect.h; y_++)
                for(int x_ = 0; x_ < rect.w; x_++)
                    char_surface.setPixel(x_, y_, surface.getPixel(rect.x + x_, rect.y + y_));
        }
}

#define TEXT_SPACING 1

gfx::Surface gfx::textToSurface(const std::string& text, Color color) {
    int width = 0;
    for(char i : text)
        width += char_surfaces[(int)(unsigned char)i].getWidth() + TEXT_SPACING;
    
    if(width == 0)
        width = 1;
    
    Surface text_surface;
    text_surface.createEmpty(width, 16);
    int curr_x = 0;
    for(char i : text) {
        text_surface.draw(curr_x, 0, char_surfaces[(int)(unsigned char)i], color);
        curr_x += char_surfaces[(int)(unsigned char)i].getWidth() + TEXT_SPACING;
    }
    return text_surface;
}

int gfx::getCharWidth(char c) {
    return char_surfaces[(int)(unsigned char)c].getWidth();
}
