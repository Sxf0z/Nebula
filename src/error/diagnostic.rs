use crate::lexer::Span;
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub source: String,
    pub severity: Severity,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}
impl Diagnostic {
    pub fn new(message: impl Into<String>, span: Span, source: &str, severity: Severity) -> Self {
        Self {
            message: message.into(),
            span,
            source: source.to_string(),
            severity,
        }
    }
    pub fn format(&self) -> String {
        let mut output = String::new();
        let lines: Vec<&str> = self.source.lines().collect();
        let line_num = self.span.line;
        if line_num == 0 || line_num > lines.len() {
            return format!("{}: {}", self.severity_str(), self.message);
        }
        let line = lines[line_num - 1];
        let line_prefix = format!("{:>4} | ", line_num);
        output.push_str(&format!("{}: {}\n", self.severity_str(), self.message));
        output.push_str(&format!("  --> line {}:{}\n", line_num, self.span.column));
        output.push_str("     |\n");
        output.push_str(&line_prefix);
        output.push_str(line);
        output.push('\n');
        let caret_offset = 7 + self.span.column.saturating_sub(1);
        output.push_str(&format!("{}^", " ".repeat(caret_offset)));
        if self.span.length > 1 {
            output.push_str(&"~".repeat(self.span.length - 1));
        }
        output
    }
    fn severity_str(&self) -> &'static str {
        match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        }
    }
}
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}
