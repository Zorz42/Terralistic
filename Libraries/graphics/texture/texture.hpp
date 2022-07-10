#pragma once
#include "transformation.hpp"
#include "color.hpp"
#include "rectShape.hpp"
#include <string>

namespace gfx {

class Texture {
protected:
    void freeTexture();
    unsigned int gl_texture = -1;
    int width = 0, height = 0;
    _Transformation texture_normalization_transform;
public:
    void render(float scale, int x, int y, bool flipped=false, Color color={255, 255, 255}) const;
    void render(float scale, int x, int y, RectShape src_rect, bool flipped=false, Color color={255, 255, 255}) const;
    
    int getTextureWidth() const;
    int getTextureHeight() const;
    void createBlankImage(int width, int height);
    void loadFromData(const unsigned char* data, int width, int height);
    void loadFromText(const std::string& text, Color color={255, 255, 255});

    void setRenderTarget();
    const _Transformation& getNormalizationTransform() const;
    unsigned int getGlTexture() const;
    
    ~Texture();
};

void resetRenderTarget();

};
