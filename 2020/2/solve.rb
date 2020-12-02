# frozen_string_literal: true

input = ARGF.readlines.map { |line| line.split(': ') }

def check_password_a(policy, password)
  range, letter = policy.split(' ')
  min, max = range.split('-').map(&:to_i)

  letters = password.chars.each_with_object({}) do |char, memo|
    memo[char] = (memo[char] || 0) + 1
  end

  letters[letter] && letters[letter] >= min && letters[letter] <= max
end

def check_password_b(policy, password)
  indexes, letter = policy.split(' ')
  x, y = indexes.split('-').map(&:to_i)
  (password[x - 1] == letter) ^ (password[y - 1] == letter)
end

puts check_password_a('1-3 a', 'abcde')
puts check_password_a('1-3 b', 'cdefg')
puts check_password_a('2-9 c', 'ccccccccc')

total_valid_passwords_a = input
  .map { |(policy, password)| check_password_a(policy, password) ? 1 : 0 }
  .sum

puts total_valid_passwords_a

puts check_password_b('1-3 a', 'abcde')
puts check_password_b('1-3 b', 'cdefg')
puts check_password_b('2-9 c', 'ccccccccc')

total_valid_passwords_b = input
  .map { |(policy, password)| check_password_b(policy, password) ? 1 : 0 }
  .sum

puts total_valid_passwords_b
