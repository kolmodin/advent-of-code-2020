
input = open("../inputs/day02.txt") do f
    read(f, String)
end

struct Record
    from::Int64
    to::Int64
    chr::Char
    pwd::String
end

format_re = r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$"

function parseLine(ln)
    # 1-3 a: abcde
    capts = match(format_re, ln).captures
    return Record(
        parse(Int64, capts[1]),
        parse(Int64, capts[2]),
        capts[3][1],
        capts[4],
    )

end

function count_valid(is_valid::Function, records::Array{Record})::Int64
    return sum([
        1 for record in records if is_valid(record)
    ])
end

records = [ parseLine(ln) for ln in split(input, "\n") ]

valid1 = count_valid(records) do record
    matches = count(c -> c == record.chr, record.pwd)
    record.from <= matches <= record.to
end

valid2 = count_valid(records) do record
    ok1 = record.pwd[record.from] == record.chr
    ok2 = record.pwd[record.to] == record.chr
    ok1 != ok2
end

println("Part 1: $(valid1)")
println("Part 2: $(valid2)")
