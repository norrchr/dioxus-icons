pub(crate) struct IndentationWriter {
    buf: Vec<String>,
    indent: String,
    level: usize,
}

impl IndentationWriter {
    pub fn new() -> Self {
        Self::new_with_indent(4)
    }

    pub fn new_with_indent(spaces: usize) -> Self {
        Self {
            buf: Vec::new(),
            indent: " ".repeat(spaces),
            level: 0,
        }
    }

    pub fn indent(&mut self) {
        self.level += 1;
    }

    pub fn dedent(&mut self) {
        self.level -= 1;
    }

    pub fn write_line(&mut self, line: impl ToString) {
        self.buf.push(format!("{}{}", self.indent.repeat(self.level), line.to_string()));
    }

    pub fn indent_after_line(&mut self, line: impl ToString) {
        self.write_line(line);
        self.indent();
    }

    pub fn dedent_before_line(&mut self, line: impl ToString) {
        self.dedent();
        self.write_line(line);
    }
}

impl ToString for IndentationWriter {
    fn to_string(&self) -> String {
        self.buf.join("\n")
    }
}