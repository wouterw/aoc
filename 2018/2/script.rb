# frozen_string_literal: true

box_ids = File.read('input.txt').strip.split

# Part 1

def checksum(box_ids)
  twice = 0
  thrice = 0

  box_ids.each do |box_id|
    letter_frequency_map = letter_frequencies(box_id)
    twice += 1 if letter_frequency_map.find { |k, v| v == 2 }
    thrice += 1 if letter_frequency_map.find { |k, v| v == 3 }
  end

  twice * thrice
end

def letter_frequencies(string)
  string.chars.reduce({}) do |acc, e|
    acc[e] = (acc[e] || 0) + 1
    acc
  end
end

test_box_ids = [
  "abcdef",
  "bababc",
  "abbcde",
  "abcccd",
  "aabcdd",
  "abcdee",
  "ababab",
]

puts checksum(test_box_ids) == 12
puts checksum(box_ids)

# Part 2

def distance(left, right)
  left.chars.zip(right.chars).reduce(0) do |distance, letters|
    distance += 1 if letters.first != letters.last
    distance
  end
end

def common_letters(left, right)
  left.chars.zip(right.chars).reduce([]) do |acc, letters|
    acc << letters.first if letters.first == letters.last
    acc
  end
end

def correct_boxes(box_ids)
  box_ids.each do |left|
    box_ids.each do |right|
      return common_letters(left, right).join if distance(left, right) == 1
    end
  end
end

test_box_ids_2 = [
  "abcde",
  "fghij",
  "klmno",
  "pqrst",
  "fguij",
  "axcye",
  "wvxyz",
]

puts correct_boxes(test_box_ids_2)
puts correct_boxes(test_box_ids_2) == 'fgij'
puts correct_boxes(box_ids)