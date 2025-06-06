use crate::cpu::instructions::macros::implement_store_instruction;
use crate::{
    cpu::state::{InstructionExecutor, InstructionState},
    memory::{LoadOps, MemAccessSize, MemoryProcessor, StoreOps},
    riscv::Instruction,
};
use nexus_common::cpu::{Processor, Registers};

pub struct SbInstruction {
    rs1: u32,
    rs2: u32,
    imm: u32,
}

implement_store_instruction!(SbInstruction, MemAccessSize::Byte);

#[cfg(test)]
mod tests {
    use nexus_common::error::MemoryError;

    use super::*;
    use crate::cpu::state::Cpu;
    use crate::memory::{LoadOp, VariableMemory, RW};
    use crate::riscv::{BuiltinOpcode, Instruction, Opcode, Register};

    fn setup_memory() -> VariableMemory<RW> {
        VariableMemory::<RW>::default()
    }

    #[test]
    fn test_sb_positive_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0x7F);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SB), 1, 2, 0);
        let instruction = SbInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1000, MemAccessSize::Byte).unwrap(),
            LoadOp::Op(MemAccessSize::Byte, 0x1000, 0x7F)
        );
    }

    #[test]
    fn test_sb_negative_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0xFFFFFF80); // -128 in two's complement

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SB), 1, 2, 1);
        let instruction = SbInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1001, MemAccessSize::Byte).unwrap(),
            LoadOp::Op(MemAccessSize::Byte, 0x1001, 0x80)
        );
    }

    #[test]
    fn test_sb_max_negative_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0xFFFFFFFF); // -1 in two's complement

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SB), 1, 2, 2);
        let instruction = SbInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1002, MemAccessSize::Byte).unwrap(),
            LoadOp::Op(MemAccessSize::Byte, 0x1002, 0xFF)
        );
    }

    #[test]
    fn test_sb_overflow() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, u32::MAX);
        cpu.registers.write(Register::X2, 0xAA);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SB), 1, 2, 1);
        let instruction = SbInstruction::decode(&bare_instruction, &cpu.registers);

        let result = instruction.memory_write(&mut memory);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MemoryError::AddressCalculationOverflow
        ));
    }

    // TODO: depending on the memory model, we need to test out of bound memory access
}
