input = open("../inputs/day03.txt") do f
    read(f, String)
end

function makeMap(inp)
    lines = split(inp, "\n")
    height = length(lines)
    width = length(lines[1])
    m = fill(' ', height, width)
    for y in 1:height
        for (x, c) in enumerate(lines[y])
            m[y,x] = c
        end
    end
    m
end

function runSlope(m, dy, dx)::Int64
    width = size(m, 2)
    height = size(m, 1)
    x = 1 + dx
    y = 1 + dy
    count = 0
    while y <= height
        if m[y,x] == '#'
            count += 1
        end
        x = mod(x + dx, 1:width)
        y += dy
    end
    count
end

m = makeMap(input)
println("Part 1: ", runSlope(m, 1, 3))

slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

println("Part 2: ", prod([runSlope(m, dy, dx) for (dx, dy) in slopes]))