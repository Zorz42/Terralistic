//
//  frameLengthMeasurer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef frameLengthMeasurer_h
#define frameLengthMeasurer_h

namespace framerateRegulator {

void regulateFramerate();
inline unsigned short frame_length = 0;
inline unsigned short fps_limit = 0;

}

#endif /* frameLengthMeasurer_h */
