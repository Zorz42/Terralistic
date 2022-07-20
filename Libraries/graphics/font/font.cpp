#include "font.hpp"

void gfx::quitFont() {
    delete font_texture;
}

bool fontColEmpty(const gfx::Surface& surface, int x, int y) {
    for(int i = 0; i < 16; i++)
        if(surface.getPixel(x, y + i).a != 0)
            return false;
    
    return true;
}

void gfx::loadFont(const Surface& surface) {
    if(surface.getWidth() != 256 || surface.getHeight() != 256)
        throw Exception("Font surface must be 256x256");
    font_texture = new Texture;
    font_texture->loadFromSurface(surface);
    
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
            
            font_rects[y * 16 + x] = rect;
        }
}
