pub fn inline_diff_md(before: &str, after: &str) -> String {
    let before_lines = before.lines().collect::<Vec<_>>();
    let after_lines = after.lines().collect::<Vec<_>>();
    let mut out = String::from("# Diff\n\n");

    let max_len = before_lines.len().max(after_lines.len());
    for idx in 0..max_len {
        match (before_lines.get(idx), after_lines.get(idx)) {
            (Some(a), Some(b)) if a == b => {
                out.push_str("  ");
                out.push_str(a);
                out.push('\n');
            }
            (Some(a), Some(b)) => {
                out.push_str("- ");
                out.push_str(a);
                out.push('\n');
                out.push_str("+ ");
                out.push_str(b);
                out.push('\n');
            }
            (Some(a), None) => {
                out.push_str("- ");
                out.push_str(a);
                out.push('\n');
            }
            (None, Some(b)) => {
                out.push_str("+ ");
                out.push_str(b);
                out.push('\n');
            }
            (None, None) => {}
        }
    }

    out
}
