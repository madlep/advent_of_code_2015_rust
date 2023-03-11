pub fn part1(data: &str) -> String {
    parser::parse(data)
        .iter()
        .map(|pair| pair.parsed_diff())
        .sum::<usize>()
        .to_string()
}

pub fn part2(data: &str) -> String {
    parser::parse(data)
        .iter()
        .map(|pair| pair.encoded_diff())
        .sum::<usize>()
        .to_string()
}

type CodeString = String;
type ParsedString = Vec<char>;

#[derive(Debug)]
pub struct CodeParsedPair {
    code: CodeString,
    parsed: ParsedString,
}

impl CodeParsedPair {
    fn parsed_diff(&self) -> usize {
        self.code.len() - self.parsed.len()
    }

    fn encoded(&self) -> Vec<char> {
        let result_iter = self.code.chars().flat_map(|c| match c {
            '\\' => vec!['\\', '\\'],
            '\"' => vec!['\\', '\"'],
            other => vec![other],
        });

        let mut result: Vec<char> = Vec::with_capacity(self.code.len() * 2);
        result.push('\"');
        result.extend(result_iter);
        result.push('\"');
        result
    }

    fn encoded_diff(&self) -> usize {
        self.encoded().len() - self.code.len()
    }
}

mod parser {
    use super::*;

    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::anychar,
        combinator::{consumed, map, map_opt, value, verify},
        multi::{fold_many0, separated_list0},
        sequence::{delimited, preceded},
        IResult,
    };

    pub fn parse(s: &str) -> Vec<CodeParsedPair> {
        let (rest, code_parsed_pairs) = code_parsed_pairs(s).unwrap();
        assert!(rest.is_empty());
        code_parsed_pairs
    }

    fn code_parsed_pairs(s: &str) -> IResult<&str, Vec<CodeParsedPair>> {
        let mut p = separated_list0(tag("\n"), code_parsed_pair);
        p(s)
    }

    fn code_parsed_pair(s: &str) -> IResult<&str, CodeParsedPair> {
        let p = consumed(parsed_string);
        let mut p = map(p, |(code, parsed)| CodeParsedPair {
            code: code.to_string(),
            parsed,
        });
        p(s)
    }

    fn parsed_string(s: &str) -> IResult<&str, ParsedString> {
        let parsed_body = fold_many0(
            body_char,
            || Vec::with_capacity(64),
            |mut parsed, c| {
                parsed.push(c);
                parsed
            },
        );
        let mut p = delimited(tag("\""), parsed_body, tag("\""));
        p(s)
    }

    fn body_char(s: &str) -> IResult<&str, char> {
        let mut p = alt((lower_alpha, escaped_slash, escaped_dquote, escaped_hex));
        p(s)
    }

    fn lower_alpha(s: &str) -> IResult<&str, char> {
        let mut p = verify(anychar, |c| c >= &'a' && c <= &'z');
        p(s)
    }

    fn escaped_slash(s: &str) -> IResult<&str, char> {
        let mut p = value('\\', tag(r#"\\"#));
        p(s)
    }

    fn escaped_dquote(s: &str) -> IResult<&str, char> {
        let mut p = value('\"', tag(r#"\""#));
        p(s)
    }

    fn escaped_hex(s: &str) -> IResult<&str, char> {
        let digit_parser = preceded(tag("\\x"), take(2usize));
        let mut p = map_opt(digit_parser, |digits| {
            let n = u32::from_str_radix(digits, 16).ok()?;
            char::from_u32(n)
        });

        p(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_escaped_backslash() {
        let code = r#""abc\\def""#;
        let pairs = parser::parse(code);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].code, code);
        assert_eq!(pairs[0].parsed, vec!['a', 'b', 'c', '\\', 'd', 'e', 'f']);
    }

    #[test]
    fn parses_escaped_double_quote() {
        let code = r#""abc\"def""#;
        let pairs = parser::parse(code);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].code, code);
        assert_eq!(pairs[0].parsed, vec!['a', 'b', 'c', '\"', 'd', 'e', 'f']);
    }

    #[test]
    fn parses_escaped_hexadecimal() {
        let code = r#""abc\x41def""#;
        let pairs = parser::parse(code);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].code, code);
        assert_eq!(pairs[0].parsed, vec!['a', 'b', 'c', 'A', 'd', 'e', 'f']);
    }
}
