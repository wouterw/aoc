# frozen_string_literal: true

def memory_game(numbers, turns)
  tally = Hash[numbers[0..-2].map.with_index { |n, i| [n, i + 1] }]

  (numbers.length..(turns - 1)).each do |i|
    prev = tally[numbers.last]
    prev = prev.nil? ? 0 : i - prev
    tally[numbers.last] = i
    numbers << prev
  end

  numbers.last
end

def assert(actual, expected)
  raise "#{actual} != #{expected}" if actual != expected
end

assert memory_game([0, 3, 6], 2020), 436
assert memory_game([1, 3, 2], 2020), 1
assert memory_game([2, 1, 3], 2020), 10
assert memory_game([1, 2, 3], 2020), 27
assert memory_game([2, 3, 1], 2020), 78
assert memory_game([3, 2, 1], 2020), 438
assert memory_game([3, 1, 2], 2020), 1836

assert memory_game([0, 3, 6], 30_000_000), 175_594
assert memory_game([1, 3, 2], 30_000_000), 2578
assert memory_game([2, 1, 3], 30_000_000), 3_544_142
assert memory_game([1, 2, 3], 30_000_000), 261_214
assert memory_game([2, 3, 1], 30_000_000), 6_895_259
assert memory_game([3, 2, 1], 30_000_000), 18
assert memory_game([3, 1, 2], 30_000_000), 362

puzzle_input = [0, 1, 5, 10, 3, 12, 19]

puts speak(puzzle_input, 2020)
puts speak(puzzle_input, 30_000_000)
