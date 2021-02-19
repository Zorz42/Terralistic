//
//  init.h
//  Terralistic
//
//  Created by Jakob Zorz on 19/02/2021.
//

#ifndef init_hpp
#define init_hpp

#define CONCAT_(x,y) x##y
#define CONCAT(x,y) CONCAT_(x,y)
#define INIT_SCRIPT init::registerInitFunction CONCAT(init_register, __LINE__) ([] {
#define INIT_SCRIPT_END });


namespace init {

typedef void(*initFunction)();
struct registerInitFunction {
    registerInitFunction(initFunction function);
};

void initModules();

}

#endif /* init_h */
