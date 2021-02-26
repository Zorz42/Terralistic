//
//  init.h
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#ifndef init_hpp
#define init_hpp

#define INIT_SCRIPT init::registerInitFunction UNIQUE_NAME(init_register) ([] {
#define INIT_SCRIPT_END return true;});
#define INIT_ASSERT(condition) if(!condition) return false


namespace init {

typedef bool(*initFunction)();
struct registerInitFunction {
    registerInitFunction(initFunction function);
};

void initModules();

}

#endif /* init_hpp */
