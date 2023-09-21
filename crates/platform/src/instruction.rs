/// 指令类型

mod instruction_memory;
trait Instruction{
    /// 定义数据类型
    type DataType;
    /// 定义偏移
    const OFFSET:usize=std::mem::align_of::<usize>();

    /// 定义执行函数
    fn execute(self,data:Self::DataType);
}
