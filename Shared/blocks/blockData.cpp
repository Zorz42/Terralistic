#include "blockData.hpp"

dataDeliverer::dataDeliverer() {
    functions.push_back(&deliverDefaultData);
    names.emplace_back("default");
    functions.push_back(&deliverFurnaceData);
    names.emplace_back("furnace");
}