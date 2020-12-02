# frozen_string_literal: true

def react(polymer)
  chars = polymer.chars

  i = 0

  while i < chars.size
    current_unit = chars[i]
    next_unit = chars[i + 1]

    if opposite_polarity(current_unit, next_unit)
      chars.delete_at(i)
      chars.delete_at(i)

      i = [0, i - 1].max

      next
    end

    i += 1
  end

  chars.join
end

def opposite_polarity(left, right)
  (char_capitalized(left) && left.downcase == right) ||
    (!char_capitalized(left) && left.upcase == right)
end

def char_capitalized(char)
  char == char.upcase
end

def unique_units(polymer)
  polymer.chars.map(&:downcase).uniq
end

def smallest_polymer(polymer)
  polymers = unique_units(polymer).map do |unit|
    reduced_polymer = polymer_without_unit(polymer, unit)
    react(reduced_polymer)
  end

  polymers.map(&:size).min
end

def polymer_without_unit(polymer, unit)
  polymer_chars = polymer.chars
  polymer_chars.delete(unit.downcase)
  polymer_chars.delete(unit.upcase)
  polymer_chars.join
end

# Input

input = File.read('input.txt').strip

# Part 1 Test

polymer = react('dabAcCaCBAcCcaDA')

puts polymer == 'dabCBAcaDA'
puts polymer.size == 10

# Part 1

# puts input.size
# output = react(input)
# puts output.size

# Part 2 Test

polymer = smallest_polymer('dabAcCaCBAcCcaDA')

puts polymer
puts polymer == 4

# Part 2

puts smallest_polymer(input)
