function command_give(arguments, executor)
    if #arguments < 1 then
        return "Command 'give' requires at least one argument."
    end
    
    if #arguments > 3 then
        return "Command 'give' takes at most three arguments."
    end
    
    player = executor
    item_name = arguments[1]
    
    item = terralistic_get_item_id_by_name(item_name)
    if item == nil then
        return "Item '" .. item_name .. "' not found."
    end
    
    amount = 1
    if #arguments >= 2 then
        amount = tonumber(arguments[2])
        if amount == nil then
            return "Argument 2 must be a number."
        end
    end
    
    if #arguments >= 3 then
        player = terralistic_get_player_by_name(arguments[3])
    end
    
    if player == nil then
        return "Player '" .. arguments[3] .. "' not found."
    end
    
    terralistic_give_item(player, item, amount)
    
    return "Gave item"
end

function describe_command_give() 
    return 
[[Gives the player an item.
Usage: give <item> [amount] [player] - amount defaults to 1, player defaults to the executor.]]
end

function command_water(arguments, executor)
    if #arguments < 2 then
        return "Command 'water' requires at least two arguments."
    end

    if #arguments > 3 then
        return "Command 'water' takes at most three arguments."
    end

    x = tonumber(arguments[1])
    y = tonumber(arguments[2])

    if x == nil or y == nil then
        return "Arguments 1 and 2 must be numbers."
    end

    level = 100
    if #arguments >= 3 then
        level = tonumber(arguments[3])
        if level == nil then
            return "Argument 3 must be a number."
        end
    end

    terralistic_set_liquid(x, y, liquids.water, level)

    return "Placed water at " .. x .. ", " .. y .. "."
end

function describe_command_water()
    return
[[Places water in the world, which then flows on its own.
Usage: water <x> <y> [level] - level is how full the block is, from 0 to 100, and defaults to 100.]]
end

function command_stop(arguments, executor)
    if #arguments ~= 0 then
        return "Command 'stop' does not take any arguments."
    end
    
    terralistic_stop_server()
    return "Stopping the server..."
end

function describe_command_stop()
    return "Stops the server."
end