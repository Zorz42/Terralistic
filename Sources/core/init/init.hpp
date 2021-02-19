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
#define REGISTER_INIT_FUNC init::registerInitFunction CONCAT(init_register, __LINE__) ([](void) {
#define REGISTER_INIT_FUNC_END });


namespace init {

typedef void(*initFunction)();
struct registerInitFunction {
    registerInitFunction(initFunction function);
};

}

#endif /* init_h */
