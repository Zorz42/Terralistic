#include "font.hpp"
#include "rectShape.hpp"

static gfx::Surface font_surface;
static gfx::RectShape font_rects[256];

bool fontColEmpty(const gfx::Surface& surface, int x, int y) {
    for(int i = 0; i < 16; i++)
        if(surface.getPixel(x, y + i).a != 0)
            return false;
    
    return true;
}

void gfx::loadFont(const Surface& surface) {
    font_surface = surface;
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
            
            font_rects[y * 16 + x] = rect;
        }
}

#define TEXT_SPACING 1

gfx::Surface gfx::textToSurface(const std::string& text, Color color) {
    int width = 0;
    for(char i : text)
        width += font_rects[(int)(unsigned char)i].w + TEXT_SPACING;
    
    if(width == 0)
        width = 1;
    
    Surface text_surface;
    text_surface.createEmpty(width, 16);
    int curr_x = 0;
    for(char i : text) {
        for(int x = 0; x < font_rects[(int)(unsigned char)i].w; x++)
            for(int y = 0; y < 16; y++) {
                Color c = font_surface.getPixel(font_rects[(int)(unsigned char)i].x + x, font_rects[(int)(unsigned char)i].y + y);
                c.r *= (float)color.r / 255;
                c.g *= (float)color.g / 255;
                c.b *= (float)color.b / 255;
                c.a *= (float)color.a / 255;
                text_surface.setPixel(x + curr_x, y, c);
            }
        
        curr_x += font_rects[(int)(unsigned char)i].w + TEXT_SPACING;
    }
    return text_surface;
}

int gfx::getCharWidth(char c) {
    return font_rects[(int)(unsigned char)c].w;
}
