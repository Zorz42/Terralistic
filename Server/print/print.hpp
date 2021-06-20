//
//  print.hpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#ifndef print_hpp
#define print_hpp

#include <string>

namespace print {

void info(const std::string& text);
void warning(const std::string& text);
void error(const std::string& text);

}

#endif /* print_hpp */
