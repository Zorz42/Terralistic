#include "worldInfo.hpp"

void WorldInfo::render() {
    gfx::RectShape((int)(x * (float)gfx::getWindowWidth()), (int)(y * (float)gfx::getWindowHeight()), width, height).render({100, 100, 100, 255});
    text.render();
}