---@diagnostic disable-next-line: lowercase-global
array = {
    -- Return the value of index
    get = function(obj, index)
        -- Normalize indexing to 0, lua indexing starts with 1.
        local item = obj.inner[index + 1]
        array.snapshot(obj, { index }, {})

        return item;
    end,

    -- Overwrite the index with value
    set = function(obj, index, value)
        -- Normalize indexing to 0, lua indexing starts with 1.
        obj.inner[index + 1] = value
        array.snapshot(obj, {}, { index })
    end,

    -- Return sublist from start_index (inclusive) to end_index (exclusive)
    get_slice = function(obj, start_index, end_index)
        return table.unpack(obj.inner, start_index + 1, end_index)
    end,

    length = function(obj)
        return #obj.inner;
    end,

    swap = function(obj, index1, index2)
        obj.inner[index1 + 1], obj.inner[index2 + 1] = obj.inner[index2 + 1], obj.inner[index1 + 1]
        array.snapshot(obj, { index1, index2 }, { index1, index2 })
    end,

    snapshot = function(obj, accesses, writes)
        local element = {
            snapshot = { table.unpack(obj.inner) },
            accesses = accesses,
            writes = writes,
        }

        -- print("snapshot = [" .. table.concat(element.snapshot, ", ") .. "]")
        -- print("accesses = [" .. table.concat(element.accesses, ", ") .. "]")
        -- print("writees  = [" .. table.concat(element.writes, ", ") .. "]")
        -- print()

        table.insert(obj.history, element)
    end
}

return array
