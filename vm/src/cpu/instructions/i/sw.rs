use crate::cpu::instructions::macros::implement_store_instruction;
use crate::{
    cpu::{
        registerfile::RegisterFile,
        state::{Cpu, InstructionExecutor},
    },
    error::{Result, VMError},
    memory::{MemAccessSize, Memory, MemoryProcessor},
    riscv::Instruction,
};

pub struct SwInstruction {
    rd: u32,
    rs1: u32,
    imm: u32,
}

implement_store_instruction!(SwInstruction, MemAccessSize::Word);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::state::Cpu;
    use crate::memory::Memory;
    use crate::riscv::{Instruction, InstructionType, Opcode, Register};

    fn setup_memory() -> Memory {
        Memory::default()
    }

    #[test]
    fn test_sw_positive_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0x7FFFFFFF);

        let bare_instruction = Instruction::new(Opcode::SW, 2, 1, 0, InstructionType::SType);
        let instruction = SwInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();

        assert_eq!(
            memory.read(0x1000, MemAccessSize::Word).unwrap(),
            0x7FFFFFFF
        );
    }

    #[test]
    fn test_sw_negative_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0x80000000); // -2147483648 in two's complement

        let bare_instruction = Instruction::new(Opcode::SW, 2, 1, 4, InstructionType::SType);
        let instruction = SwInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();

        assert_eq!(
            memory.read(0x1004, MemAccessSize::Word).unwrap(),
            0x80000000
        );
    }

    #[test]
    fn test_sw_unaligned_address() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1001); // Unaligned address
        cpu.registers.write(Register::X2, 0xABCDEF01);

        let bare_instruction = Instruction::new(Opcode::SW, 2, 1, 0, InstructionType::SType);
        let instruction = SwInstruction::decode(&bare_instruction, &cpu.registers);

        let result = instruction.memory_write(&mut memory);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VMError::UnalignedMemoryWrite(0x1001)
        ));
    }

    #[test]
    fn test_sw_overflow() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, u32::MAX);
        cpu.registers.write(Register::X2, 0xDEADBEEF);

        let bare_instruction = Instruction::new(Opcode::SW, 2, 1, 1, InstructionType::SType);
        let instruction = SwInstruction::decode(&bare_instruction, &cpu.registers);

        let result = instruction.memory_write(&mut memory);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            VMError::AddressCalculationOverflow
        ));
    }

    // TODO: depending on the memory model, we need to test out of bound memory access
}
