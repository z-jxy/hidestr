
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Token {
    pub comment_ident: String,
    pub script_block_open_ident: String,
    pub script_block_close_ident: String,
    pub block_open_left: char,
    pub block_open_right: char,
    pub block_close_left: char,
    pub block_close_right: char,
    pub comment_ident_len: usize,
    pub comment_tokens: (char, Option<char>),
}

impl Token {
    pub fn new(comment_ident: &str, sb_open_ident: &str, sb_end_ident: &str) -> Self {
        let (script_open_left, script_open_right) = (sb_open_ident.chars().nth(0).unwrap(), sb_open_ident.chars().nth(1).unwrap());

        Self {
            comment_ident: String::from(comment_ident),
            script_block_open_ident: String::from(sb_open_ident),
            script_block_close_ident: String::from(sb_end_ident),
            block_open_left: script_open_left,
            block_open_right: script_open_right,
            block_close_left: sb_end_ident.chars().nth(0).unwrap(),
            block_close_right: sb_end_ident.chars().nth(1).unwrap(),
            comment_ident_len: comment_ident.len(),
            comment_tokens: Self::tokens_from_ident(comment_ident.len(), comment_ident),
        }
    }
    pub fn tokens_from_ident(ident_len: usize, ident: &str) -> (char, Option<char>) {
        if ident_len == 2 {
            let (left, right) = (ident.chars().nth(0).unwrap(), ident.chars().nth(1).unwrap());
            return (left, Some(right))
        } 
        (ident.chars().nth(0).unwrap(), None)
    }

    pub fn get_token_type(s: &str) -> Token {
        let t = TOKENS::from_extension(s);
        println!("Using token: {:?}", t);
        return t.to_token();
    }

    pub fn callback(&self, (chars, i, char_len): (&Vec<char>, usize, usize) ) -> bool {
        let t = TOKENS::from_token(self);
        match t {
            TOKENS::POWERSHELL => {
                if i + 2 < char_len && chars[i + 1] == 'r' { 
                    // this means it's a #require statement
                    return false; 
                }
                true
            },
            // TODO: more testing to see they are any checks that can be done here
            TOKENS::C => true,
        }
    }
}


#[derive(Debug)]
pub enum TOKENS {
    POWERSHELL, // Powershell
    C, // C, C++, C#, etc. (should work for any language that uses C-style comments)
}

impl TOKENS {
    fn to_token(&self) -> Token {
        match self {
            TOKENS::POWERSHELL => Token::new("#", "<#", "#>"),
            TOKENS::C => Token::new("//", "/*", "*/"),
        }
    }

    fn from_extension(ext: &str) -> Self {
        match ext.trim_start_matches('.') {
            "ps1" => TOKENS::POWERSHELL,
            "cs" | "c" | "cpp" | "h" | "hpp" | "cc" => TOKENS::C,
            _ => panic!("[-] Unknown file extension: {}", ext),
        }
    }

    fn from_token(token: &Token) -> Self {
        match token.comment_ident.as_str() {
            "#" => TOKENS::POWERSHELL,
            "//" => TOKENS::C,
            _ => panic!("[-] Unknown comment token: {}", token.comment_ident),
        }
    }
}