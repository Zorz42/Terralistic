#include <iostream>
#include "constantValues.hpp"

void ConstantIntegerType::print(Value* value, int depth) {
    ConstantInteger* const_int = (ConstantInteger*)value;
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << const_int->value << std::endl;
}

Value* ConstantIntegerType::parse(const Token*& curr_token) {
    if(curr_token->type != TokenType::CONSTANT_INTEGER)
        return nullptr;
    
    ConstantInteger* const_int = new ConstantInteger(this);
    const_int->value = curr_token->const_int;
    curr_token++;
    
    return const_int;
}
