#include "virtualMachine.hpp"

void VirtualMachine::registerAnInstructionType(InstructionType *type) {
    type->id = (int)instruction_types.size();
    instruction_types.push_back(type);
}

const std::vector<Instruction*>& VirtualMachine::getInstructions() {
    return instructions;
}

void VirtualMachine::addInstructions(const std::vector<Instruction*>& instructions_to_add) {
    for(Instruction* instruction : instructions_to_add)
        instructions.push_back(instruction);
}
