#pragma once
#include <string>
#include "transformation.hpp"
#include "color.hpp"
#include "rectShape.hpp"
#include "surface.hpp"

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
    void loadFromSurface(const Surface& surface);

    void setRenderTarget();
    const _Transformation& _getNormalizationTransform() const;
    unsigned int _getGlTexture() const;
    
    ~Texture();
};

void resetRenderTarget();

};
