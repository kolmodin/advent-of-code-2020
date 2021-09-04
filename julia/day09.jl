nums = open("inputs/day09.txt") do f
    parse.(Int64, readlines(f))
end

function solve(nums, step=25)::Union{Int64,Nothing}
    start = 1
    for i in start + step:length(nums)
        slice = view(nums, i:(i + step))
        last = nums[i + step]
        search = function ()
            for a in slice
                for b in slice
                    if last == a + b
                        return true
                    end
                end
            end
            return false
        end
        if !search()
            return last
        end
    end
end


function solve2(nums, target)::Union{Int64,Nothing}
    for start in 1:length(nums)
        mx = nums[start]
        mn = nums[start]
        acc = nums[start]
        for i in (start + 1):length(nums)
            acc += nums[i]
            mx = max(mx, nums[i])
            mn = min(mn, nums[i])
            if acc > target
                break
            end
            if acc == target
                return mn + mx
            end
        end
    end
end

function main()
    part1 = solve(nums)
    println("Part 1: ", part1)
    println("Part 2: ", solve2(nums, part1))
end

main()