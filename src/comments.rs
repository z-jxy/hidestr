
pub fn clean_comments(lines: Vec<String>) -> String {
    return post_process(remove_comments(lines));
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

fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut open = false;
    let mut ret = vec![];
    let mut cache = String::new();

    source.iter().for_each(|x| {
        cache += &parse_chars(&x.chars().collect::<Vec<char>>(), 0, &mut open);
        if !open && !cache.is_empty() {
            ret.push(cache.clone());
            cache.clear();
        }
    });

    return ret;
}

fn parse_chars(s: &Vec<char>, k: usize, open: &mut bool) -> String {
    let (mut i, char_len) = (k, s.len());
    let mut ret = String::new();
    
    if i == char_len { return ret }
    
    if *open {
        while i + 1 < char_len {
            if s[i] == '*' && s[i + 1] == '/' {
                *open = false;
                return parse_chars(s, i + 2, open);
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
        
        if s[i] == '\'' || s[i] == '"' { in_string = !in_string }

        if i + 1 < char_len && s[i] == '/' && s[i + 1] == '/' { return ret }
        if i + 1 < char_len && s[i] == '/' && s[i + 1] == '*' {
            *open = true;
            ret += &parse_chars(s, i + 2, open);
            return ret
        }
        ret.push(s[i]);
        i += 1;
    };

    
    ret
}
