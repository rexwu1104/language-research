#[derive(Debug)]
pub struct Token {
    pub raw: String,
    pub line: usize,
    pub position: (usize, usize),
    pub ty: TokenType
}

#[derive(Debug)]
pub enum TokenType {
    Char,
    String,
    Atom,
    Identifier,
    Number,
    Symbol,
    NewLine
}

macro_rules! generate_token {
    ($type:ident, $line:expr, $position:expr, $raw:expr) => {Token {
        raw: $raw,
        line: $line,
        position: $position,
        ty: TokenType::$type
    }};
}

macro_rules! match_type {
    ($type:expr, $num:literal, $($pattern:pat => $block:expr),+) => {match $type {
        0 | $num => (),
        $($pattern => $block,)+
        _ => ()
    }};
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut buffer = String::new();
    let mut line = 0;
    let mut position = (0, 0);

    let mut string_start = '\0';
    let mut comment_start = false;
    let mut comment = false;
    let mut type_num = 0;
    for c in content.chars() {
        position.1 += 1;
        if comment && c != '\n' {
            continue;
        } else if comment && c == '\n' {
            comment = false;
        }

        match c {
            '"' | '\'' | '`' => if string_start == '\0' { string_start = c },
            '\r' => continue,
            '\n' => { line += 1; position = (0, 0); if buffer.len() == 0 { continue } },
            '/' => if comment_start {
                comment = true;
                comment_start = false;
                position.0 = position.1;
                string_start = '\0';
                type_num = 0;
                buffer.clear();
                continue;
            } else { comment_start = true }
            _ => ()
        }

        if string_start != '\0' {
            buffer.push(c);
            
            if buffer.len() >= 2 && buffer.chars().take(buffer.chars().count() - 1).last() == Some('\\') {}
            else if buffer.len() != 1 && buffer.chars().last() == Some(string_start) {
                tokens.push(match string_start {
                    '"'  => generate_token!(String, line, position, buffer.clone()),
                    '\'' => generate_token!(Char  , line, position, buffer.clone()),
                    '`'  => generate_token!(Atom  , line, position, buffer.clone()),
                    _ => todo!()
                });

                position.0 = position.1;
                string_start = '\0';
                buffer.clear();
            }
        } else {
            match c {
                ' ' | '\t' => if type_num != 0 {
                    match_type!(type_num, 1,
                    2 => tokens.push(generate_token!(Identifier, line, position, buffer.clone())),
                    3 => tokens.push(generate_token!(Number, line, position, buffer.clone())),
                    4 => tokens.push(generate_token!(Symbol, line, position, buffer.clone())));

                    position.0 = position.1;
                    type_num = 0;
                    buffer.clear();
                    continue;
                } else { continue },
                '\n' => if type_num != 0 {
                    match_type!(type_num, 1,
                    2 => tokens.push(generate_token!(Identifier, line, position, buffer.clone())),
                    3 => tokens.push(generate_token!(Number, line, position, buffer.clone())),
                    4 => tokens.push(generate_token!(Symbol, line, position, buffer.clone())));

                    position.0 = position.1;
                    type_num = 0;
                    buffer.clear();
                    tokens.push(generate_token!(NewLine, line, position, buffer.clone()));
                    continue;
                },
                'a'..='z' | 'A'..='Z' | '_' => if type_num != 2 && type_num != 0 {
                    match_type!(type_num, 2,
                    3 => tokens.push(generate_token!(Number, line, position, buffer.clone())),
                    4 => tokens.push(generate_token!(Symbol, line, position, buffer.clone())));

                    position.0 = position.1;
                    type_num = 2;
                    buffer.clear();
                } else { type_num = 2 },
                '0'..='9' => if type_num != 3 && type_num != 0 {
                    match_type!(type_num, 3,
                    2 => tokens.push(generate_token!(Identifier, line, position, buffer.clone())),
                    4 => tokens.push(generate_token!(Symbol    , line, position, buffer.clone())));

                    position.0 = position.1;
                    type_num = 3;
                    buffer.clear();
                } else { type_num = 3 },
                '\0'..='\u{20}' | '\u{127}' => continue,
                _ => if type_num != 4 && type_num != 0 {
                    match_type!(type_num, 4,
                    2 => tokens.push(generate_token!(Identifier, line, position, buffer.clone())),
                    3 => tokens.push(generate_token!(Number    , line, position, buffer.clone())));

                    position.0 = position.1;
                    type_num = 4;
                    buffer.clear();
                } else {
                    type_num = 4;
                    if buffer.len() == 1 {
                        tokens.push(generate_token!(Symbol    , line, position, buffer.clone()));
                        position.0 = position.1;
                        type_num = 4;
                        buffer.clear();
                    }
                }
            }

            buffer.push(c);
        }
    }

    if buffer.len() != 0 {
        match type_num {
            2 => tokens.push(generate_token!(Identifier, line, position, buffer.clone())),
            3 => tokens.push(generate_token!(Number    , line, position, buffer.clone())),
            4 => tokens.push(generate_token!(Symbol    , line, position, buffer.clone())),
            _ => ()
        }
    }

    tokens
}