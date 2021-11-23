#pragma once

#include <vector>

class Instruction;

class InstructionType {
public:
    int id;
    virtual void print(Instruction* instruction);
};

class Instruction {
public:
    Instruction(InstructionType* type) : type(type) {}
    InstructionType* const type;
};

class VirtualMachine {
    std::vector<InstructionType*> instruction_types;
public:
    void registerAnInstructionType(InstructionType* type);
};
