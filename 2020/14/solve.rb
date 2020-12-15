# frozen_string_literal: true

def parse(program)
  program.split("\n").map do |line|
    match = line.match(/^(\w+)(\[(\d+)\])? = (.*)$/)

    case match[1]
    when 'mask'
      [:mask, match[4]]
    when 'mem'
      [:mem, match[3].to_i, match[4].to_i]
    end
  end
end

def apply_bitmask(mask, decimal)
  ones = mask.tr('X', '0').to_i(2)
  zeros = mask.tr('X', '1').to_i(2)

  (decimal | ones) & zeros
end

def execute_a(input)
  program = parse(input)

  mask = nil
  mem = Hash.new(0)

  program.each do |instruction|
    instr, address_or_mask, decimal = instruction

    case instr
    when :mask
      mask = address_or_mask
    when :mem
      mem[address_or_mask] = apply_bitmask(mask, decimal)
    end
  end

  mem
end

def part1(input)
  memory = execute_a(input)
  memory.values.sum
end

def execute_b(input)
  program = parse(input)

  mask = nil
  mem = Hash.new(0)

  program.each do |instruction|
    instr, address_or_mask, decimal = instruction

    case instr
    when :mask
      mask = address_or_mask
    when :mem
      addresses = bitmask_permutations(apply_bitmask_string(mask, decimal_to_36_bits(address_or_mask)))
      addresses.each { |addr| mem[addr] = decimal }
    end
  end

  mem
end

def apply_bitmask_string(mask, decimal)
  mask.chars.zip(decimal.chars).map do |m, d|
    case m
    when '0' then d
    when '1' then '1'
    when 'X' then 'X'
    end
  end.join
end

def bitmask_permutations(mask)
  return mask unless mask.include?('X')

  s1 = mask.sub('X', '1')
  s2 = mask.sub('X', '0')

  [bitmask_permutations(s1), bitmask_permutations(s2)].flatten
end

def decimal_to_36_bits(decimal)
  decimal.to_s(2).rjust(36, '0')
end

def part2(input)
  memory = execute_b(input)
  memory.values.sum
end

sample_program_a = <<~TXT
  mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
  mem[8] = 11
  mem[7] = 101
  mem[8] = 0
TXT

sample_program_b = <<~TXT
  mask = 000000000000000000000000000000X1001X
  mem[42] = 100
  mask = 00000000000000000000000000000000X0XX
  mem[26] = 1
TXT

pp part1(sample_program_a) == 165
pp part2(sample_program_b) == 208

puzzle_program = ARGF.read

pp part1(puzzle_program)
pp part2(puzzle_program)
