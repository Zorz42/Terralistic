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
    ~furnaceData() override{};
    void save(std::vector<char>& data, unsigned long& index) override;
    void load(const char*& iter) override;
    int getSavedSize() override{return 24;}
};


struct dataDeliverer{
    std::vector<defaultData* (*)()> functions;
    std::vector<std::string> names;
    static defaultData* deliverDefaultData(){return nullptr;}
    static defaultData* deliverFurnaceData(){return (defaultData*)(new furnaceData);}
    dataDeliverer();
};