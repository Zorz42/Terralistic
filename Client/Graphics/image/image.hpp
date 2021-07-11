#pragma once
class Image {
public:
    void setTexture(void* texture_);
    [[nodiscard]] inline void* getTexture() const { return texture; }
    ~Image();
    bool free_texture = true, flipped = false;
    [[nodiscard]] unsigned short getTextureWidth() const;
    [[nodiscard]] unsigned short getTextureHeight() const;
    void clear();
    void setAlpha(unsigned char alpha);
protected:
    void freeTexture();
    void* texture = nullptr;
};
