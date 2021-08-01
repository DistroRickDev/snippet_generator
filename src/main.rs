mod file_parser;
mod tokenizer;
mod file_exporter;

fn main() {
    let mut _new_parser =  file_parser::FileParser::new("mystruct.dl".to_string());
    _new_parser.get_raw_data_from_file();
    let my_tokens = tokenizer::tokenize(&_new_parser.get_raw_data());
    tokenizer::print_tokens(&my_tokens);
    file_exporter::openfile();
}
