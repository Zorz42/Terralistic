#pragma once
#include "liquids.hpp"
#include "inventory.hpp"

namespace BlockTypes {
    extern BlockType dirt;
    extern BlockType stone_block;
    extern BlockType grass_block;
    extern BlockType stone;
    extern BlockType wood;
    extern BlockType leaves;
    extern BlockType sand;
    extern BlockType snowy_grass_block;
    extern BlockType snow_block;
    extern BlockType ice_block;
    extern BlockType iron_ore;
    extern BlockType copper_ore;
    extern BlockType grass;
}

namespace LiquidTypes {
    inline LiquidType water(/*name*/"water", /*flow_time*/100, /*speed_multiplier*/0.5, /*color*/{0, 92, 230, 150});
}

namespace ItemTypes {
    inline ItemType stone      (/*name*/"stone",       /*max_stack*/99, /*places*/&BlockTypes::stone      );
    inline ItemType dirt       (/*name*/"dirt",        /*max_stack*/99, /*places*/&BlockTypes::dirt       );
    inline ItemType stone_block(/*name*/"stone_block", /*max_stack*/99, /*places*/&BlockTypes::stone_block);
    inline ItemType wood_planks(/*name*/"wood_planks", /*max_stack*/99, /*places*/&BlockTypes::air        );
    inline ItemType iron_ore   (/*name*/"iron_ore",    /*max_stack*/99, /*places*/&BlockTypes::iron_ore   );
    inline ItemType copper_ore (/*name*/"copper_ore",  /*max_stack*/99, /*places*/&BlockTypes::copper_ore );
    inline ItemType fiber      (/*name*/"fiber",       /*max_stack*/99, /*places*/&BlockTypes::air        );
    inline ItemType hatchet    (/*name*/"hatchet",     /*max_stack*/1,  /*places*/&BlockTypes::air        );
}

void addContent(Blocks* blocks, Liquids* liquids, Items* items, Recipes* recipes);
