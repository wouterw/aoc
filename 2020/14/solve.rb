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

def execute(input)
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
  memory = execute(input)
  memory.values.sum
end

sample_program = <<~TXT
  mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
  mem[8] = 11
  mem[7] = 101
  mem[8] = 0
TXT

pp part1(sample_program) == 165

puzzle_program = ARGF.read

pp part1(puzzle_program)
