#pragma once
#include "expression.hpp"

class ConstantIntegerType : public ValueType {
public:
    void print(Value* value, int depth) override;
    Value* parse(const Token*& curr_token) override;
};

namespace ValueTypes {
    inline ConstantIntegerType constant_integer;
};

class ConstantInteger : public Value {
public:
    ConstantInteger() : Value(ValueTypes::constant_integer.id) {}
    int value;
};
