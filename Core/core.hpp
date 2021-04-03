//
//  essential.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/02/2021.
//

#ifndef essential_hpp
#define essential_hpp

#define JOIN_(a, b) a##b
#define JOIN(a, b) JOIN_(a, b)

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
#include "blockEngine.hpp"

#else
//#warning Essential header has been imported multiple times
#endif /* essential_hpp */
