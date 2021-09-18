#include "updater.hpp"
#include <SFML/Network.hpp>

void checkForUpdatesMacOS();
void checkForUpdatesWindows();
void checkForUpdatesLinux();

void checkForUpdates() {
#ifdef _WIN32
    checkForUpdatesWindows();
#endif
    
#ifdef __APPLE__
    checkForUpdatesMacOS();
#endif
    
#ifdef __linux__
    checkForUpdatesLinux();
#endif
}

void checkForUpdatesMacOS() {
     
}

void checkForUpdatesWindows() {
    
}

void checkForUpdatesLinux() {
    
}
