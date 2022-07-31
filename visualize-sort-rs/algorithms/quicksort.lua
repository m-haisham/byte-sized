require "scripts.array"

local function sort(values)
    local function partition(values, low, high)
        local pivot = array.get(values, high)
        print(string.format("high = %d, pivot = %s", high, tostring(pivot)))

        local i = low - 1

        for j = low, high - 1, 1 do
            print(string.format("[j = %d]", j))
            if array.get(values, j) < pivot then
                i = i + 1
                array.swap(values, i, j)
            end
        end

        array.swap(values, i + 1, high)
        return i + 1
    end

    local function quicksort(values, low, high)
        print(high)
        if low < high then
            local pivot = partition(values, low, high)

            quicksort(values, low, pivot - 1)
            quicksort(values, pivot + 1, high)
        end
    end

    quicksort(values, 0, array.length(values) - 1)
end

return {
    name = "Quick Sort",
    sort = sort,
}
