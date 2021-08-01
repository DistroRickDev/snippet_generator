use std::collections::HashMap;

/**
 * TODO: implement it inside a struct with level access
 */

#[derive(Debug, PartialEq, Eq, Hash)]
enum ValidTokens {
    LBracket,
    RBracket,
    LSquareBracket,
    RSquareBracket,
    LParenthesis,
    RParenthesis,
    Colon,
    Hashtag,
    FSlash,
    Asterisk,
    Underscore,
    At
}

struct ValidTokenMapper{
    pub valid_token_map: HashMap<ValidTokens, String> 
}

impl ValidTokenMapper{
    fn new() -> ValidTokenMapper {
        let mut new_token_mapper = ValidTokenMapper {
            valid_token_map : HashMap::new()
        };

        new_token_mapper.valid_token_map.insert(ValidTokens::LBracket,          "{".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::RBracket,          "}".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::LSquareBracket,    "[".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::RSquareBracket,    "]".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::LParenthesis,      "(".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::RParenthesis,      ")".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::Colon,             ":".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::Hashtag,           "#".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::FSlash,            "{".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::Asterisk,          "*".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::Underscore,        "_".to_string(),);
        new_token_mapper.valid_token_map.insert(ValidTokens::At,                "@".to_string(),);
        
        return new_token_mapper;
    }
}

fn validate_token(ch: &String) -> bool {
    let map = ValidTokenMapper::new().valid_token_map; 
    for (_key, val) in &map {
        println!("Checking {} and {}", ch, val);
        if val == ch {
            return true;
        }
    }
    false
}

pub fn tokenize(raw_str: &String) -> Vec<String> {
    // Creates readonly vector to hold all chars of string
    let placeholder : Vec<char> = raw_str.chars().collect();

    // Place holder for string token
    let mut temp_string = String::new();

    // Return Vec<String>
    let mut string_vec : Vec<String> = Vec::new();

    for ch in placeholder{
        if ch == ' ' || ch == '\n'{
            if !temp_string.is_empty() {
                string_vec.push(temp_string.clone());
                temp_string.clear();
            }
            continue;
        }
        temp_string.push(ch);
    }
    string_vec.push(temp_string);
    return string_vec;
}

pub fn print_tokens(tokens: &Vec<String>){
    for token in tokens{
        println!("TOKEN: {}", token);
    }
}