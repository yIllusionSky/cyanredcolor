use super::Instruction;

/// 对内存操作

impl Instruction for crate::memory::InstructionUnit{
    type DataType = crate::memory::MemoryUnit;
    fn execute(self,data:Self::DataType) {
        
    }
}