# frozen_string_literal: true

def parse(input)
  arr = input.split("\n").map(&:to_i).sort
  [0, *arr, arr.max + 3]
end

def permutations(joltages = [], i = 0, memo = {})
  return 1 if i == joltages.size - 1

  return memo[i] if memo.include?(i)

  paths = 0

  (i + 1...joltages.size).each do |j|
    paths += permutations(joltages, j, memo) if joltages[j] - joltages[i] <= 3
  end

  memo[i] = paths
end

def differences(joltages)
  (0..joltages.size - 2).each_with_object(Hash.new(0)) do |i, memo|
    difference = joltages[i + 1] - joltages[i]
    memo[difference] += 1
  end
end

def part1(input)
  joltages = parse(input)

  h = differences(joltages)

  h[1] * h[3]
end

def part2(input)
  joltages = parse(input)

  permutations(joltages)
end

demo_input = <<~TXT
  16
  10
  15
  5
  1
  11
  7
  19
  6
  12
  4
TXT

larger_demo_input = <<~TXT
  28
  33
  18
  42
  31
  14
  46
  20
  48
  47
  24
  23
  49
  45
  19
  38
  39
  11
  1
  32
  25
  35
  8
  17
  7
  9
  4
  2
  34
  10
  3
TXT

puzzle_input = ARGF.read

puts part1(demo_input)
puts part1(larger_demo_input)
puts part1(puzzle_input)
puts ''
puts part2(demo_input)
puts part2(larger_demo_input)
puts part2(puzzle_input)
