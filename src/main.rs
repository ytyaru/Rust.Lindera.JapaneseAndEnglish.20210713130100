use lindera::tokenizer::Tokenizer;

fn main() {
    println!("Hello, world!");
    let text = "日本語とenglishが混在mixしたテキストですが何か問題でも？ This is a pen. I am Tom. I'm Andy. What's this? rustで実行する。あれを動かした。C#で書く。";
    println!("{}", text);
    println!("{:?}", tokenize_japanese(text));
    println!("{:?}", tokenize_japanese_2(text));
    println!("{:?}", tokenize_japanese_3(text));
    println!("{:?}", tokenize_japanese_4(text));
}

// https://github.com/mattico/elasticlunr-rs/blob/master/src/pipeline.rs
// https://github.com/mattico/elasticlunr-rs/blob/master/src/lang/ja.rs
// https://github.com/lindera-morphology/lindera/blob/master/lindera/src/tokenizer.rs
// アルファベットが対象外になってしまう
pub fn tokenize_japanese(text: &str) -> Vec<String> {
    let mut tokenizer = Tokenizer::new("decompose", "");
    tokenizer
        .tokenize(text)
        .into_iter()
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("助詞") | Some("助動詞") | Some("記号") | Some("UNK") => None,
            _ => Some(tok.text.to_string()),
        })
        .collect()
}
// 英単語も含めたい。Some("UNK")に入っているようだ。除外したらたしかに英単語が入った。しかし1文字のものや半角記号まで含まれてしまう
pub fn tokenize_japanese_2(text: &str) -> Vec<String> {
    let mut tokenizer = Tokenizer::new("decompose", "");
    tokenizer
        .tokenize(text)
        .into_iter()
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("助詞") | Some("助動詞") | Some("記号") => None,
            _ => Some(tok.text.to_string()),
        })
        .collect()
}
// 英単語も含めたい。ただし１文字なら対象外にする。
pub fn tokenize_japanese_3(text: &str) -> Vec<String> {
    let mut tokenizer = Tokenizer::new("decompose", "");
    tokenizer
        .tokenize(text)
        .into_iter()
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("助詞") | Some("助動詞") | Some("記号") => None,
            Some("UNK") if tok.text.len() < 2 => None,
            _ => Some(tok.text.to_string()),
        })
        .collect()
}
// 動詞は基本形にしたかったができなかった。
pub fn tokenize_japanese_4(text: &str) -> Vec<String> {
    let mut tokenizer = Tokenizer::new("decompose", "");
    for token in tokenizer.tokenize(text) {
        println!("{:?}\t{}", token.detail, token.text);
    }
    /*
    tokenizer
        .tokenize(text)
        .into_iter()
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("動詞") if tok.detail.get(6).is_some() => Some(tok.detail.get(6).unwrap()), // 動詞は基本形にして返す。連用形などでなく。
            _ => Some(tok),
        .collect()
    */
    tokenizer
        .tokenize(text)
        .into_iter()
        .filter_map(|tok| match tok.text.to_lowercase().as_str() {
            " " | "." | "'" | "!" | "\"" | "$" | "%" | "&" | "(" | ")" | "=" | "^" | "~" | "|" | "@" | "`" | "[" | "]" | "{" | "}" | "+" | ";" | ":" | "*" | ">" | "," | "<" | "/" | "?" | "\\" | "_" | "|" | "a" | "am" | "is" | "was" | "this" | "that" => None,
            _ => Some(tok),
        })
//        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
//            Some("動詞") if tok.text.as_str() == "し" => None,
//            Some("動詞") if tok.text.as_str() == "する" => None,
//        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
//            Some("動詞") if tok.detail.get(6).is_some() => Some(tok.detail.get(6).unwrap()), // 動詞は基本形にして返す。連用形などでなく。
//            _ => Some(tok),
        .filter_map(|tok| match tok.detail.get(0).map(|d| d.as_str()) {
            Some("助詞") | Some("助動詞") | Some("記号") => None,
            Some("UNK") if tok.text.len() < 2 => None,
//            Some("UNK") if tok.text.as_str() => 
//            Some("UNK") if 1 < tok.text.len() && tok.text.to_lowercase().chars().all(|c| c.is_digit(36) || c == '_').as_str() => 
//            Some("UNK") if 1 < tok.text.len() && tok.text.to_lowercase().as_str() => 
//            Some("動詞") => Some(match tok.detail.get(6).map(|d| d.as_str())), // 動詞は基本形にして返す。連用形などでなく。
//            Some("動詞") if tok.detail.get(6).is_some() => Some(tok.detail.get(6).unwrap()),
//            Some("動詞") => Some(tok.detail.get(6).unwrap()),
            _ => Some(tok.text.to_string()),
        })
        .collect()
}
