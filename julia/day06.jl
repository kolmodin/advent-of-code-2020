input = open("../inputs/day06.txt") do f
    read(f, String)
end

groups = [
    [
        Set([c for c in ln])
        for ln in split(group, "\n")
    ]
    for group in split(input, "\n\n")
]

part1 = sum([length(union(group...)) for group in groups])
println("Part 1: ", part1)

part2 = sum([length(intersect(group...)) for group in groups])
println("Part 2: ", part2)
