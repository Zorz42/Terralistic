//
//  essential.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/02/2021.
//

#ifndef essential_hpp
#define essential_hpp

#ifndef FILENAME
#error Macro variable FILENAME has not been defined
#endif
#ifndef NAMESPACE
#error Macro variable NAMESPACE has not been defined
#endif

#define CONCAT(a, b) a##b
#define JOIN(a, b) CONCAT(a, b)

#else
#warning Essential header has been imported multiple times
#endif /* essential_hpp */
