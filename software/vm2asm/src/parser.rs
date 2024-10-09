#[derive(Debug, Clone)]
pub struct LineSource {
    pub tokens: Vec<String>,
    pub line: usize,
}

pub struct Parser;

impl Parser {
    pub fn parse(source: String) -> Vec<LineSource> {
        let source = source
            .split("\n")
            .map(str::to_string)
            .collect::<Vec<String>>();
        let mut line_sources: Vec<LineSource> = Vec::default();
        for (i, line) in source.into_iter().enumerate() {
            if let Some(line) = Self::remove_whtiespace(line) {
                line_sources.push(LineSource {
                    tokens: Self::generate_tokens(line),
                    line: i + 1,
                });
            }
        }
        line_sources
    }

    fn generate_tokens(source: String) -> Vec<String> {
        source
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect::<Vec<String>>()
    }

    fn remove_whtiespace(mut line: String) -> Option<String> {
        // Strip out comments part
        if line.contains("//") {
            line = line
                .split("//")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[0]
                .clone();
        }
        // Trim all remaining whitespace
        line = line.trim().to_string();
        // If its an empty line, we skip it
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_remove_whitespace() {
        let source = String::from("\n \n  //\n//no way hose ");
        let line_sources = Parser::parse(source);
        assert_eq!(line_sources.len(), 0);
    }
    #[test]
    fn token_before_comment_remove_whitespace() {
        let source = String::from("@10//wow what is thi");
        let line_sources = Parser::parse(source);
        assert_eq!(line_sources[0].tokens[0], "@10".to_string());

        let source = String::from("(LOOP) //no");
        let line_sources = Parser::parse(source);
        assert_eq!(line_sources[0].tokens[0], "(LOOP)".to_string());

        let source = String::from("push local 2 //no");
        let line_sources = Parser::parse(source);
        assert_eq!(line_sources[0].tokens.len(), 3);
    }
    #[test]
    fn generate_tokens() {
        let source = String::from("pop  local    2");
        let tokens = Parser::generate_tokens(source);
        assert_eq!(
            tokens,
            vec![
                String::from("pop"),
                String::from("local"),
                String::from("2")
            ]
        );

        let source = String::from("add");
        let tokens = Parser::generate_tokens(source);
        assert_eq!(tokens, vec![String::from("add"),]);

        let source = String::from("push constant 3");
        let tokens = Parser::generate_tokens(source);
        assert_eq!(
            tokens,
            vec![
                String::from("push"),
                String::from("constant"),
                String::from("3")
            ]
        );
    }
    #[test]
    fn parse() {
        let source = String::from("push constant 2\npush   constant   3 //wow comment\nadd\n\n  ");
        let line_souces = Parser::parse(source);
        assert_eq!(line_souces.len(), 3);
        assert_eq!(line_souces[1].tokens.len(), 3);
        assert_eq!(
            line_souces[0].tokens,
            vec!["push".to_string(), "constant".to_string(), "2".to_string()]
        );
        assert_eq!(
            line_souces[1].tokens,
            vec!["push".to_string(), "constant".to_string(), "3".to_string()]
        );
        assert_eq!(line_souces[2].tokens, vec!["add".to_string()]);
    }
}
