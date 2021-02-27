//
//  dev.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef dev_h
#define dev_h

#include "print.hpp"

// enabling developer mode can slow down the server, but it will show any errors that program made, like accessing out of array, i would reccomend adding -D DEVELOPER_MODE to compiler flags to enable it

#ifdef DEVELOPER_MODE
#define ASSERT(expression, message) if(!(expression)) { print::error(message); assert(expression); }
#define IF_DEV(x) if(x)
#else
#define ASSERT(x, message) x
#define IF_DEV(x) x; if(x && false)
#endif

#endif /* dev_h */
