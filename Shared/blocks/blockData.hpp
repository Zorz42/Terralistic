#pragma once
#include <vector>
#include <string>
#include "blocks.hpp"
#include "items.hpp"

struct FurnaceData : DefaultData{
    int burn_time = 0;
    int heat = 0;
    ItemStack fuel = {nullptr, 0};
    ItemStack heated_items = {nullptr, 0};
    ~FurnaceData() override{};
    void save(std::vector<char>& data, unsigned long& index) override;
    void load(const char*& iter) override;
    int getSavedSize() override{return 24;}
};


struct dataDeliverer{
    std::vector<DefaultData* (*)()> functions;
    std::vector<std::string> names;
    static DefaultData* deliverDefaultData(){return nullptr;}
    static DefaultData* deliverFurnaceData(){return (DefaultData*)(new FurnaceData);}
    dataDeliverer();
};
