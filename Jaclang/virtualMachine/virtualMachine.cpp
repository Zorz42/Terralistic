#include "virtualMachine.hpp"

void VirtualMachine::registerAnInstructionType(InstructionType *type) {
    type->id = (int)instruction_types.size();
    instruction_types.push_back(type);
}
