#pragma once

#include <vector>

class Instruction;

class InstructionType {
public:
    int id;
    virtual void print(Instruction* instruction) = 0;
};

class Instruction {
public:
    Instruction(InstructionType* type) : type(type) {}
    InstructionType* const type;
};

class VirtualMachine {
    std::vector<InstructionType*> instruction_types;
    std::vector<Instruction*> instructions;
public:
    void registerAnInstructionType(InstructionType* type);
    const std::vector<Instruction*>& getInstructions();
    void addInstructions(const std::vector<Instruction*>& instructions_to_add);
};
