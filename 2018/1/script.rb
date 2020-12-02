# frozen_string_literal: true

require 'set'

input = File.read('input.txt').strip

# PART 1

def parse_frequencies(string)
  string.split.map { |e| Integer(e) }
end

def resulting_frequency(frequencies)
  frequencies.reduce(&:+)
end

puts resulting_frequency(parse_frequencies(input))

# PART 2

def first_repeating_frequency(frequencies)
  i = 0
  sum = 0
  found = false
  seen_frequencies = Set.new

  while !found do
    frequency = frequencies[i % frequencies.size]

    sum += frequency

    found = seen_frequencies.include?(sum)

    seen_frequencies << sum

    i += 1
  end

  sum
end

test = "+7\n+7\n-2\n-7\n-4\n"

puts first_repeating_frequency(parse_frequencies(test)) == 14
puts first_repeating_frequency(parse_frequencies(input))
