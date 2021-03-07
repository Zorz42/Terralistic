//
//  clickEvents.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 07/03/2021.
//

#ifndef clickEvents_hpp
#define clickEvents_hpp

#include "playerHandler.hpp"

namespace clickEvents {

struct clickEvents {
    void (*rightClickEvent)(blockEngine::block*, playerHandler::player*) = nullptr;
    void (*leftClickEvent)(blockEngine::block*, playerHandler::player*) = nullptr;
};

inline std::vector<clickEvents> click_events;

}

#endif /* clickEvents_hpp */
