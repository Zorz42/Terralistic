function register_recipes()
    terralistic_print("registering recipes...")

    -- HATCHET
    terralistic_register_recipe(items.hatchet, 1, { items.stone, items.branch, items.fiber }, { 1, 1, 2 })

    -- HAMMER
    terralistic_register_recipe(items.hammer, 1, { items.stone, items.branch, items.fiber }, { 1, 1, 2 })
    
    -- PICKAXE
    terralistic_register_recipe(items.pickaxe, 1, { items.stone, items.branch, items.fiber }, { 1, 1, 2 })

    -- WOOD_PLANK_WALL FROM WOOD_PLANKS
    terralistic_register_recipe(items.wood_plank_wall, 4, { items.wood_planks }, { 1 })

    -- WOOD_PLANKS FROM WOOD_PLANK_WALL
    terralistic_register_recipe(items.wood_planks, 1, { items.wood_plank_wall }, { 4 })

    -- WOOD_PLANKS TO TORCH
    terralistic_register_recipe(items.torch, 1, { items.wood_planks }, { 1 })
end