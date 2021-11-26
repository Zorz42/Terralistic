#include "virtualMachine.hpp"

void VirtualMachine::registerAnInstructionType(InstructionType *type) {
    type->id = (int)instruction_types.size();
    instruction_types.push_back(type);
}

const std::vector<Instruction*>& VirtualMachine::getInstructions() {
    return instructions;
}

void VirtualMachine::addInstructions(const std::vector<Instruction*>& instructions_to_add) {
    instructions.insert(instructions.end(), instructions_to_add.begin(), instructions_to_add.end());
}
