//! 内存

use std::{ptr::NonNull, marker::PhantomData, alloc::Layout};
use super::instruction;

mod instruction_memory;
mod data_memory;
/// 内存单元
/// 
enum MemoryUnit{
    /// 指令类型
    instruction(instruction_memory::InstructionUnit),
    /// 数据块类型
    data(data_memory::DataUnit),
    /// 字节类型
    byte(u8),
}

/// 内存空间
/// 
struct MemorySpace{
    // 内存指针位置
    ptr:NonNull<MemoryUnit>,
    // 内存对齐信息
    layout:Layout,
}

#[allow(dead_code)]
impl MemorySpace{
    /// 创建一片新的内存空间
    /// 
    /// `align_size`：对齐大小
    /// 
    /// `size`：内存块数量 
    fn new(unit_count:usize,align_size:usize)->color_eyre::Result<MemorySpace>{

        let layout=std::alloc::Layout::from_size_align(unit_count,align_size)?;
        
        let ptr=unsafe{std::alloc::alloc(layout) as *mut MemoryUnit};
        if ptr.is_null(){
            Err(color_eyre::eyre::eyre!("内存分配失败"))
        }else{
            let ptr=NonNull::new(ptr).ok_or(color_eyre::eyre::eyre!("内存分配失败"))?;
            Ok(MemorySpace {ptr , layout })
        }
    }
}

impl Drop for MemorySpace{
    /// 释放内存
    /// 如果[`std::alloc::dealloc`]释放失败，会造成unsafe
    fn drop(&mut self) {
        // 获取原始指针
        let ptr=self.ptr.as_ptr() as *mut u8;

        unsafe{std::alloc::dealloc(ptr,self.layout)};
    }
}