use std::{collections::BTreeMap, io::Write};

pub struct D2Context<W: Write> {
    writer: W,
    pub bom: BTreeMap<String, usize>,
}

impl<W: Write> D2Context<W> {
    pub fn new(w: W) -> Self {
        let mut w = w;
        writeln!(&mut w, "direction: right").unwrap();
        Self {
            bom: BTreeMap::default(),
            writer: w,
        }
    }

    pub fn add_node(&mut self, node: &str) {
        writeln!(self.writer, "{}", node).unwrap();
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        writeln!(self.writer, "{} -> {}", from, to).unwrap();
    }

    pub fn writeln(&mut self, s: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{s}")
    }
}

pub struct NodeGuard<'a, W: Write> {
    pub ctx: &'a mut D2Context<W>,
}

impl<'a, W: Write> NodeGuard<'a, W> {
    pub fn new(
        ctx: &'a mut D2Context<W>,
        _node_type: &str,
        node_name: &str,
        description: &str,
    ) -> Self {
        ctx.writeln(&format!(
            "{node_name}: {{
label:\"{description}\"
"
        ))
        .unwrap();
        Self { ctx }
    }

    pub fn ctx(&mut self) -> &mut D2Context<W> {
        self.ctx
    }
}

impl<W: Write> Drop for NodeGuard<'_, W> {
    fn drop(&mut self) {
        // Drop 中不能返回 Result，所以只能忽略错误
        let _ = self.ctx.writeln("}");
    }
}
