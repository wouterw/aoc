# frozen_string_literal: true

n = 7

chars = 1 + 2 * (n - 1)

(0..n - 1).each_with_index.each do |i|
  w = (chars - (1 + (2 * i))) / 2
  print ' ' * w
  print '*' * (1 + 2 * i)
  puts ''
end

(n / 3).times.each do
  print ' ' * ((chars - 3) / 2)
  print '***'
  puts ''
end
