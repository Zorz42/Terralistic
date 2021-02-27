//
//  dev.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#ifndef dev_hpp
#define dev_hpp

// enabling developer mode can slow down the game, but it will show any errors that program made, like accessing out of array, i would reccomend adding -D DEVELOPER_MODE to compiler flags to enable it

#ifdef DEVELOPER_MODE
#define ASSERT(expression, message) if(!(expression)) { std::cout << message << std::endl; SDL_assert(expression); }
#define IF_DEV(x) if(x)
#else
#define ASSERT(x, message) x
#define IF_DEV(x) x; if(false)
#endif

#endif /* dev_hpp */
