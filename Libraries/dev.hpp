//
//  dev.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/02/2021.
//

#ifndef dev_h
#define dev_h

// enabling developer mode can slow down the game, but it will show any errors that program made, like accessing out of array

#define DEVELOPER_MODE

#ifdef DEVELOPER_MODE
#define IF_DEV(x) if(x)
#else
#define IF_DEV(x) if(false)
#endif

#endif /* dev_h */
