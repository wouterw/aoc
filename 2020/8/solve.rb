# frozen_string_literal: true

require 'set'

def parse(string)
  string.split("\n").map(&:strip).map do |s|
    match = s.match(/^(\w{3}) ([+-]\d+)$/)
    [match[1], match[2].to_i]
  end
end

def execute(program)
  instruction_pointer = 0
  accumulator = 0
  exit_code = 0
  visited_instructions = Set.new

  loop do
    if instruction_pointer >= program.size
      exit_code = 0
      break
    end

    if visited_instructions.include?(instruction_pointer)
      exit_code = 1
      break
    end

    visited_instructions.add(instruction_pointer)

    operation, argument = program[instruction_pointer]

    case operation
    when 'acc'
      accumulator += argument
      instruction_pointer += 1
    when 'jmp'
      instruction_pointer += argument
    when 'nop'
      instruction_pointer += 1
    end
  end

  [exit_code, accumulator]
end

def fix_program_naively(program)
  result = -1

  program.each.with_index do |instruction, index|
    operation, argument = instruction

    modified_program = program

    if %w[jmp nop].include?(operation)
      modified_program = program.dup
      opt = operation == 'jmp' ? 'nop' : 'jmp'
      modified_program[index] = [opt, argument]
    end

    exit_code, accumulator = execute(modified_program)

    if exit_code.zero?
      result = accumulator
      break
    end
  end

  result
end

non_terminating_program_input = <<~PROGRAM
  nop +0
  acc +1
  jmp +4
  acc +3
  jmp -3
  acc -99
  acc +1
  jmp -4
  acc +6
PROGRAM

terminating_program_input = <<~PROGRAM
  nop +0
  acc +1
  jmp +4
  acc +3
  jmp -3
  acc -99
  acc +1
  nop -4
  acc +6
PROGRAM

non_terminating_program = parse(non_terminating_program_input)
accumulator = execute(non_terminating_program)
puts "terminating_program: #{accumulator}"

terminating_program = parse(terminating_program_input)
accumulator = execute(terminating_program)
puts "non_terminating_program: #{accumulator}"

puzzle_input = ARGF.read
puzzle_program = parse(puzzle_input)

accumulator_a = execute(puzzle_program)
puts "Puzzle answer a: #{accumulator_a}"

accumulator_b = fix_program_naively(puzzle_program)
puts "Puzzle answer b: #{accumulator_b}"
