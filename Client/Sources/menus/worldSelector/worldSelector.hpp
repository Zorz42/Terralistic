//
//  worldSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldSelector_hpp
#define worldSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include <iostream>

namespace worldSelector {

struct scene : public gfx::scene {
    void refresh() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
    void onMouseScroll(int distance) override;
};

}

#endif /* worldSelector_hpp */
