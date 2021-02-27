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

#define CONCAT4_(a, b, c, d) a##b##c##d
#define CONCAT4(a, b, c, d) CONCAT4_(a, b, c, d)
#define UNIQUE_NAME(tag) CONCAT4(tag, FILENAME, NAMESPACE, __LINE__)

#include <iostream>
#include <vector>
#include <random>
#include <cmath>
#include <thread>
#include <fstream>
#include <algorithm>

#include "init.hpp"
#include "events.hpp"
#include "itemEngine.hpp"
#include "inventory.hpp"
#include "terrainGenerator.hpp"
#include "packets.hpp"
#include "fileSystem.hpp"
#include "dev.hpp"

#else
#warning Essential header has been imported multiple times
#endif /* essential_hpp */
