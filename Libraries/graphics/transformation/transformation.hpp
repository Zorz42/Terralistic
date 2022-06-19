#pragma once

namespace gfx {

class _Transformation {
    float matrix[3][3] = {
        {1.f, 0.f, 0.f},
        {0.f, 1.f, 0.f},
        {0.f, 0.f, 1.f},
    };
    void applyMatrix(float applied_matrix[3][3]);
public:
    void translate(float x, float y);
    void stretch(float x, float y);
    const float* getArray() const;
    _Transformation operator*(const _Transformation& a);
};

};
