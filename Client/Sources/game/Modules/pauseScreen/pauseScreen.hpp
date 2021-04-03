//
//  pauseScreen.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/12/2020.
//

#ifndef pauseScreen_hpp
#define pauseScreen_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"

struct pauseScreen : public gfx::sceneModule<game> {
    using gfx::sceneModule<game>::sceneModule;
    void init() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
private:
    gfx::button resume_button, quit_button;
    bool paused = false;
};

#endif /* pauseScreen_hpp */
