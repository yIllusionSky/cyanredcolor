/// 指令内存单元

/// 这里是内存的指令单元
#[derive(Clone,Copy)]
#[repr(usize)]
pub enum InstructionUnit{
    // 加法
    Add,FAdd,
    // 减法
    Sub,FSub,
    // 乘法
    Mul,FMul,
    // 除法
    Div,FDiv,
    // 赋值
    Assign,
    // >
    Gt,FGt,
    // >=
    Ge,FGe,
    // <
    Lt,FLt,
    // <=
    Le,FLe,
    // ==
    Eq,FEq,
    // 循环
    While,
    // 跳转
    Jump,Dim
}