#ifndef print_hpp
#define print_hpp

#include <string>

namespace print {
    void info(const std::string& text);
    void warning(const std::string& text);
    void error(const std::string& text);
}

#endif
