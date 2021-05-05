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

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif


#endif

class pauseScreen : public gfx::sceneModule {
    gfx::button resume_button, quit_button;
    bool paused = false;
public:
    void init() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
};

#endif /* pauseScreen_hpp */
