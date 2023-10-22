tools = {}

function register_tools()
    terralistic_print("registering tools...")

    tools.axe = terralistic_register_tool("axe")

    tools.hammer = terralistic_register_tool("hammer")

    tools.pickaxe = terralistic_register_tool("pickaxe")
    
    tools.shovel = terralistic_register_tool("shovel")
end