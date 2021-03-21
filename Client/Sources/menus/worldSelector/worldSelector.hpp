//
//  worldSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef worldSelector_hpp
#define worldSelector_hpp

#include <Graphics/graphics.hpp>

namespace worldSelector {

void loop();

struct scene : public gfx::scene {
    void refresh();
    void onKeyDown(gfx::key key);
    void render();
};

}

#endif /* worldSelector_hpp */
