//
//  generatingScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef generatingScreen_hpp
#define generatingScreen_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include <thread>
#include "map.hpp"

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

struct generatingScreen : public gfx::scene {
    map& world_map;
    explicit generatingScreen(unsigned int seed, map& world_map) : seed(seed), world_map(world_map) {}
    void init() override;
    void update() override;
    void render() override;
    void stop() override;
private:
    unsigned int seed;
    std::thread thread;
    gfx::sprite loading_text;
    gfx::rect loading_bar_back{0, -LOADING_RECT_ELEVATION, (unsigned short)(LOADING_RECT_WIDTH), LOADING_RECT_HEIGHT, {100, 100, 100}, gfx::bottom},
    loading_bar{0, -LOADING_RECT_ELEVATION, 0, LOADING_RECT_HEIGHT, {255, 255, 255}, gfx::bottom};
};

#endif /* generatingScreen_hpp */
