/// Indent each line of a string by a given number of levels (1 level is 4 spaces).
pub fn indent(s: &str, level: usize) -> String {
    s.lines()
        .map(|line| format!("{: >level$}{}", "", line, level=level*4))
        .collect::<Vec<_>>()
        .join("\n")
}
