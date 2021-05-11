//
//  terrainGenerator.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 25/06/2020.
//

#ifndef terrainGenerator_hpp
#define terrainGenerator_hpp

namespace terrainGenerator {

int generateTerrainDaemon(unsigned int seed, serverMap* world_serverMap);
inline unsigned int generating_current = 0, generating_total = 6;

}

#endif /* terrainGenerator_hpp */
