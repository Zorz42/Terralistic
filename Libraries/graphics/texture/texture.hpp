#pragma once
#include <string>
#include "transformation.hpp"
#include "color.hpp"
#include "rectShape.hpp"
#include "surface.hpp"
#include "nonCopyable.hpp"

namespace gfx {

class Texture : public NonCopyable {
    void freeTexture();
    unsigned int gl_texture = -1;
    int texture_width = 0, texture_height = 0;
    _Transformation texture_normalization_transform;
    
public:
    void render(float scale, int x, int y, bool flipped=false, Color color={255, 255, 255}) const;
    void render(float scale, int x, int y, RectShape src_rect, bool flipped=false, Color color={255, 255, 255}) const;
    
    int getTextureWidth() const;
    int getTextureHeight() const;
    void loadFromSurface(const Surface& surface);
    
    const _Transformation& _getNormalizationTransform() const;
    unsigned int _getGlTexture() const;
    
    ~Texture();
};

};
