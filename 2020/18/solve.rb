# frozen_string_literal: true

def assert(expr)
  raise 'Assertion failed.' unless expr
end

# Part 1

# class Integer
#   alias times *
#   alias - times
# end

# def my_eval(expr)
#   eval expr.tr('*', '-')
# end

# assert my_eval('2 * 3 + (4 * 5)') == 26
# assert my_eval('5 + (8 * 3 + 9 + 3 * 4 * 3)') == 437
# assert my_eval('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))') == 12_240
# assert my_eval('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2') == 13_632

# puts ARGF.readlines.map { |line| my_eval(line) }.sum

# Part 2

class Integer
  alias add +
  alias + *
  alias * add
end

def my_eval(expr)
  eval expr.tr('+*', '*+')
end

assert my_eval('1 + (2 * 3) + (4 * (5 + 6))') == 51
assert my_eval('2 * 3 + (4 * 5)') == 46
assert my_eval('5 + (8 * 3 + 9 + 3 * 4 * 3)') == 1445
assert my_eval('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))') == 669_060
assert my_eval('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2') == 23_340

puts ARGF.readlines.map { |line| my_eval(line) }.sum
