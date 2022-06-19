#pragma once

namespace gfx {

class RectArray {
    int length = 0;
    std::vector<float> vertex_array;
    std::vector<float> color_array;
    std::vector<float> texture_pos_array;
    unsigned int vertex_buffer = -1, color_buffer, texture_pos_buffer;
    bool update_vertex = true, update_color = true, update_texture_vertex = true;
    
    void setVertex(int index, int x, int y);
    void setVertexColor(int index, Color color);
    void setVertexTextureCoord(int index, int x, int y);
    
public:
    void setRect(int index, RectShape rect);
    void setColor(int index, Color color);
    void setColor(int index, Color color1, Color color2, Color color3, Color color4);
    void setTextureCoords(int index, RectShape texture_coordinates);
    void render(const Texture* image=nullptr, int x=0, int y=0, bool blend_multiply=false);
    void resize(int size);
    ~RectArray();
};

};
