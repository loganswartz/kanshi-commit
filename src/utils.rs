/// Indent each line of a string by a given number of levels (1 level is 4 spaces).
pub fn indent(s: &str, level: usize) -> String {
    s.lines()
        .map(|line| format!("{: >level$}{}", "", line, level=level*4))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        let input = "foo\nbar\nbaz";
        let expected = "    foo\n    bar\n    baz";
        assert_eq!(indent(input, 1), expected);
    }

    #[test]
    fn test_multiple_indent() {
        let input = "foo\nbar\nbaz";
        let expected = "        foo\n        bar\n        baz";
        assert_eq!(indent(input, 2), expected);
    }
}
