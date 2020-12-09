# frozen_string_literal: true

require 'set'

def parse(input)
  input.split("\n").map(&:strip).map(&:to_i)
end

def valid?(numbers, sum)
  unique_set_of_numbers = Set.new(numbers)

  numbers.any? do |number|
    unique_set_of_numbers.include?(sum - number)
  end
end

def first_invalid_number(numbers, preamble = 25)
  result = nil

  numbers.each.with_index do |number, index|
    next unless index >= preamble

    window = numbers[index - preamble..index - 1]

    unless valid?(window, number)
      result = number
      break
    end
  end

  result
end

def contiguous_set(numbers, target)
  result = []

  numbers.each_with_index do |_, index|
    set = []

    numbers[index..].each do |n|
      sum = set.sum

      if sum == target && set.size > 1
        result = set
        break
      elsif sum < target
        set << n
      else
        next
      end
    end
  end

  result
end

demo_input = <<~TXT
  35
  20
  15
  25
  47
  40
  62
  55
  65
  95
  102
  117
  150
  182
  127
  219
  299
  277
  309
  576
TXT

puzzle_input = ARGF.read

demo_numbers = parse(demo_input)
puzzle_numbers = parse(puzzle_input)

demo_first_invalid_number = first_invalid_number(demo_numbers, 5)
puts "Demo answer a: #{demo_first_invalid_number}"

puzzle_first_invalid_number = first_invalid_number(puzzle_numbers, 25)
puts "Puzzle answer a: #{puzzle_first_invalid_number}"

demo_contiguous_set = contiguous_set(demo_numbers, demo_first_invalid_number)
pp demo_contiguous_set
demo_encryption_weakness = demo_contiguous_set.minmax.sum
puts "Demo answer b: #{demo_encryption_weakness}"

puzzle_contiguous_set = contiguous_set(puzzle_numbers, puzzle_first_invalid_number)
puzzle_encryption_weakness = puzzle_contiguous_set.minmax.sum
puts "Puzzle answer b: #{puzzle_encryption_weakness}"
