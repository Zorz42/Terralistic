#include <cstring>
#include "graphics-internal.hpp"

void gfx::_Transformation::applyMatrix(float applied_matrix[3][3]) {
    _Transformation applied_transformation;
    memcpy(applied_transformation.matrix, applied_matrix, sizeof(matrix));
    *this = *this * applied_transformation;
}

void gfx::_Transformation::stretch(float x, float y) {
    float applied_matrix[3][3] = {
        {x,   0.f, 0.f},
        {0.f, y,   0.f},
        {0.f, 0.f, 1.f},
    };
    applyMatrix(applied_matrix);
}

void gfx::_Transformation::translate(float x, float y) {
    float applied_matrix[3][3] = {
        {1.f, 0.f, 0.f},
        {0.f, 1.f, 0.f},
        {x,   y,   1.f},
    };
    applyMatrix(applied_matrix);
}

const float* gfx::_Transformation::getArray() const {
    return &matrix[0][0];
}

gfx::_Transformation gfx::_Transformation::operator*(const _Transformation& a) {
    _Transformation result;
    for(int x = 0; x < 3; x++)
        for(int y = 0; y < 3; y++) {
            float sum = 0;
            for(int i = 0; i < 3; i++)
                sum += matrix[i][y] * a.matrix[x][i];
            result.matrix[x][y] = sum;
        }
    return result;
}