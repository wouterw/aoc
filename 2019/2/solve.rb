# frozen_string_literal: true

def exec(program)
  memory = program.dup
  instruction_pointer = 0

  loop do
    instruction = memory[instruction_pointer...instruction_pointer + 4]
    opcode, a, b, c = instruction

    case opcode
    when 1
      memory[c] = memory[a] + memory[b]
    when 2
      memory[c] = memory[a] * memory[b]
    when 99
      break
    else

      raise "Unknown opcode: #{opcode}"
    end

    instruction_pointer += 4
  end

  memory
end

require "minitest/autorun"

class TestComputer < MiniTest::Test
  def test_program_one
    assert_equal([2, 0, 0, 0, 99], exec([1, 0, 0, 0, 99]))
  end

  def test_program_two
    assert_equal([2, 3, 0, 6, 99], exec([2, 3, 0, 3, 99]))
  end

  def test_program_three
    assert_equal([2, 4, 4, 5, 99, 9801], exec([2, 4, 4, 5, 99, 0]))
  end

  def test_program_four
    assert_equal([30, 1, 1, 4, 2, 5, 6, 0, 99], exec([1, 1, 1, 4, 99, 5, 6, 0, 99]))
  end
end

def part1(input)
  program = input.dup
  program[1] = 12
  program[2] = 2
  memory = exec(program)
  puts(memory[0])
end

def part2(input)
  program = input.dup

  (0..99).each do |noun|
    (0..99).each do |verb|
      program[1] = noun
      program[2] = verb
      output = exec(program)[0]

      if output == 19_690_720
        puts(100 * noun + verb)
      end
    end
  end
end

input = ARGF.read.split(",").map(&:to_i)
part1(input)
part2(input)
