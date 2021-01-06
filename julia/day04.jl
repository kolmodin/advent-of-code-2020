
using BenchmarkTools

input = open("../inputs/day04.txt") do f
    read(f, String)
end

all_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]

passports = [
    [ split(kv, ':')
        for ln in split(passport, "\n")
        for kv in split(ln, ' ')
    ]
    for passport in split(input, "\n\n")
]

valid1 = filter(pp -> issubset(all_fields, map(first, pp)), passports)
println("Part 1: ", length(valid1))

function parsed_in_range(s::AbstractString, lower::Int64, upper::Int64)::Bool
    val = tryparse(Int64, s)
    val !== nothing && lower <= val <= upper
end

function is_valid2(kvs)::Bool
    # Using @match here adds 1020ms of latency at startup.
    all(kvs) do (k, v)
        if k == "byr"
            return parsed_in_range(v, 1920, 2002)
        elseif k == "iyr"
            return parsed_in_range(v, 2010, 2020)
        elseif k == "eyr"
            return parsed_in_range(v, 2020, 2030)
        elseif k == "hgt"
            unit = v[end - 1:end]
            return (unit == "cm" && parsed_in_range(v[1:3], 150, 193)) || (unit == "in" && parsed_in_range(v[1:2], 59, 76))
        elseif k == "hcl"
            re = r"^#([0-9a-f]{6})$"
            return match(re, v) !== nothing
        elseif k == "ecl"
            return v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        elseif k == "pid"
            re = r"^[0-9]{9}$"
            return match(re, v) !== nothing
        else
            return true
        end
    end
end

# function is_valid2(kvs)::Bool
#     all(kvs) do (k, v)
#         @match k begin
#             "byr" => parsed_in_range(v, 1920, 2002)
#             "iyr" => parsed_in_range(v, 2010, 2020)
#             "eyr" => parsed_in_range(v, 2020, 2030)
#             "hgt" => begin
#                 unit = v[end - 1:end]
#                 (unit == "cm" && parsed_in_range(v[1:3], 150, 193)) || 
#                   (unit == "in" && parsed_in_range(v[1:2], 59, 76))
#             end
#             "hcl" => match(r"^#([0-9a-f]{6})$", v) !== nothing
#             "ecl" => v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
#             "pid" => match(r"^[0-9]{9}$", v) !== nothing
#             _ => true
#         end
#     end
# end

valid2 = length(filter(is_valid2, valid1))
println("Part 2: ", valid2)
