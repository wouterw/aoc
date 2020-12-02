# frozen_string_literal: true

def parse_claim(string)
  i, a = string.split('@')
  pt, dim = a.split(':')
  x, y = pt.strip.split(',').map(&:to_i)
  w, h = dim.strip.split('x').map(&:to_i)

  [i.strip, x, y, w, h]
end

def inches_for_claim(claim)
  _, xx, yy, w, h = claim

  inches = []

  (xx..(xx + w - 1)).each do |x|
    (yy..(yy + h - 1)).each do |y|
      inches << [x, y]
    end
  end

  inches
end

def claimed_inches(claims)
  claims.reduce({}) do |acc, claim|
    inches = inches_for_claim(claim)

    inches.each do |pt|
      acc[pt] = (acc[pt] || 0) + 1
    end

    acc
  end
end

def inches_of_overlapping_fabric(claims)
  claimed = claimed_inches(claims)
  overlapping_claims = claimed.select { |_k, v| v >= 2 }
  overlapping_claims.count
end

def intact_claim(claims)
  claimed = claimed_inches(claims)

  claims.find do |claim|
    inches_for_claim(claim).all? { |pt| claimed[pt] == 1 }
  end
end

# Input

input = File.read('input.txt').strip
claims = input.split("\n").map { |e| parse_claim(e) }

# Part 1

# test_input =
#   '#1 @ 1,3: 4x4\n' \
#   '#2 @ 3,1: 4x4\n' \
#   '#3 @ 5,5: 2x2\n'

# test_claims = test_input.strip.split('\n').map { |e| parse_claim(e) }

# puts inches_of_overlapping_fabric(test_claims) == 4

puts inches_of_overlapping_fabric(claims)

# Part 2

test_input =
  '#1 @ 1,3: 4x4\n' \
  '#2 @ 3,1: 4x4\n' \
  '#3 @ 5,5: 2x2\n'

test_claims = test_input.strip.split('\n').map { |e| parse_claim(e) }

puts intact_claim(test_claims).first == '#3'

puts intact_claim(claims).first