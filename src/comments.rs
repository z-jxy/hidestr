use crate::tokenizer::{Token};


pub fn clean_comments(lines: Vec<String>, token: &Token) -> String {
    return post_process(remove_comments(lines, token));
}


fn post_process(data: Vec<String>) -> String {
    let mut output = Vec::new();
    data.iter()
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            if x.chars().all(|x| x.is_whitespace()) {
                return;
            }
            output.push(format!("{}", x));
        });
    output.join("\n")
}


fn remove_comments(source: Vec<String>, token: &Token) -> Vec<String> {
    let mut open = false;
    let mut ret = vec![];
    let mut cache = String::new();

    source.iter().for_each(|x| {
        cache += &parse_tokens(&x.chars().collect::<Vec<char>>(), 0, &mut open, &token);
        if !open && !cache.is_empty() {
            ret.push(cache.clone());
            cache.clear();
        }
    });

    return ret;
}


fn parse_tokens(s: &Vec<char>, k: usize, open: &mut bool, token: &Token) -> String {
    let (mut i, char_len) = (k, s.len());
    let mut ret = String::new();
    
    if i == char_len { return ret }
    
    if *open {
        while i + 1 < char_len {
            if s[i] == token.block_close_left && s[i + 1] == token.block_close_right {
                *open = false;
                return parse_tokens(s, i + 2, open, token);
            }
            i += 1;
        }
        
        return ret
    }

    let mut in_string = false;


    while i < char_len {

        if in_string {
            ret.push(s[i]);
            i += 1;
            continue;
        }

        if s[i] == '\'' || s[i] == '"' { 
            in_string = !in_string;
            ret.push(s[i]);
            i += 1;
            continue;
        }

        let state = (s, i, char_len, token);

        if is_comment(state) { return ret }

        if is_comment_block(state) {
            *open = true;
            ret += &parse_tokens(s, i + 2, open, token);
            return ret
        }

        ret.push(s[i]);
        i += 1;
    };
    ret
}


fn is_comment( 
    (chars, i, char_len, token): (&Vec<char>, usize, usize, &Token) ) -> bool {
    match token.comment_tokens.1 {
        Some(right) => {
            if i + 1 < char_len && chars[i] == token.comment_tokens.0 && chars[i + 1] == right { 
                return token.callback((chars, i, char_len)) 
            }
        },
        None => {
            if i + 1 < char_len && chars[i] == token.comment_tokens.0 { 
                return token.callback((chars, i, char_len)) 
            }
        }
    }
    false
}

fn is_comment_block( 
    (s, i, char_len, token): (&Vec<char>, usize, usize, &Token) ) -> bool {
    if i + 1 < char_len && s[i] == token.block_open_left && s[i + 1] == token.block_open_right { 
        return token.callback((s, i, char_len)) 
    }
    false
}