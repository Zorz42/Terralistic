#pragma once
#include <vector>
#include <string>
#include "blocks.hpp"
#include "items.hpp"

class furnaceData : defaultData{
    int burn_time = 0;
    int heat = 0;
    ItemStack fuel = {nullptr, 0};
    ItemStack heated_items = {nullptr, 0};
public:
    int rand_int = rand();
    virtual ~furnaceData(){};
};


struct dataDeliverer{
    std::vector<defaultData* (*)()> functions;
    std::vector<std::string> names;
    static defaultData* deliverDefaultData(){return nullptr;}
    static defaultData* deliverFurnaceData(){return (defaultData*)(new furnaceData);}
    dataDeliverer();
};