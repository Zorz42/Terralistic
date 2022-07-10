#include "font.hpp"

void gfx::quitFont() {
    delete font_texture;
}

bool fontColEmpty(const unsigned char* data, int x, int y) {
    int curr_index = (256 - y) * 256 + x;
    for(int i = 0; i < 16; i++) {
        if(data[curr_index * 4 + 3] != 0)
            return false;
        curr_index -= 256;
    }
    
    return true;
}

void gfx::loadFont(const unsigned char* data) {
    font_texture = new Texture;
    font_texture->loadFromData(data, 256, 256);
    for(int y = 0; y < 16; y++)
        for(int x = 0; x < 16; x++) {
            RectShape rect(x * 16, y * 16, 16, 16);
            
            while(rect.w > 0 && fontColEmpty(data, rect.x, rect.y)) {
                rect.x++;
                rect.w--;
            }
            
            while(rect.w > 0 && fontColEmpty(data, rect.x + rect.w - 1, rect.y))
                rect.w--;
            
            if(y * 16 + x == ' ') {
                rect.x = x * 16;
                rect.w = 2;
            }
            
            font_rects[y * 16 + x] = rect;
        }
}
