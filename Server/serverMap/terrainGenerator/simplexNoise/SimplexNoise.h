/**
 * @file    SimplexNoise.h
 * @brief   A Perlin Simplex Noise C++ Implementation (1D, 2D, 3D).
 *
 * Copyright (c) 2014-2018 Sebastien Rombauts (sebastien.rombauts@gmail.com)
 *
 * Distributed under the MIT License (MIT) (See accompanying file LICENSE.txt
 * or copy at http://opensource.org/licenses/MIT)
 */
#pragma once

#include <cstddef>  // size_t
#include <cstdint>  // int32_t/uint8_t

/**
 * @brief A Perlin Simplex Noise C++ Implementation (1D, 2D, 3D, 4D).
 */
class SimplexNoise {
public:
    // 1D Perlin simplex noise
    float noise(float x);
    // 2D Perlin simplex noise
    float noise(float x, float y);
    
    /**
     * Constructor of to initialize a fractal noise summation
     */
    explicit SimplexNoise(unsigned int seed) : seed((float)seed / 4294967295.0f) {}

private:
    float seed;
};
