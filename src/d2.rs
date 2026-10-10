use std::io::Write;

use crate::ctx::Ctx;

/// 打开一个 D2 节点，析构时自动闭合。节点内部的端口在守卫存活期间写入。
pub struct NodeGuard<'a, W: Write> {
    pub ctx: &'a mut Ctx<W>,
}

impl<'a, W: Write> NodeGuard<'a, W> {
    pub fn new(ctx: &'a mut Ctx<W>, node_name: &str, label: &str) -> Self {
        ctx.writeln(&format!("{node_name}: {{\nlabel:\"{label}\"\n"))
            .unwrap();
        Self { ctx }
    }
}

impl<W: Write> Drop for NodeGuard<'_, W> {
    fn drop(&mut self) {
        // Drop 中不能返回 Result，只能忽略写错误
        let _ = self.ctx.writeln("}");
    }
}
