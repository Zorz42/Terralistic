#include "theme.hpp"
#include "shadow.hpp"
#include "blur.hpp"

void gfx::initShadow() {
    shadow_texture = new Texture;
    shadow_texture->createBlankImage(700, 700);
    Texture shadow_texture_back;
    shadow_texture_back.createBlankImage(700, 700);
    shadow_texture->setRenderTarget();
    RectShape(200, 200, 300, 300).render({0, 0, 0});
    
    _Transformation shadow_transform = shadow_texture->getNormalizationTransform();
    shadow_transform.translate(-700, 0);
    shadow_transform.stretch(2, 2);
    for(int i = 0; i < 10; i++)
        blurRectangle(RectShape(0, 0, 700, 700), GFX_SHADOW_BLUR, shadow_texture->getGlTexture(), shadow_texture_back.getGlTexture(), 700, 700, shadow_transform);
}

void gfx::quitShadow() {
    delete shadow_texture;
}
