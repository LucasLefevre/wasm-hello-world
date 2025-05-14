use tokenizer::tokenizer::{parallel_tokenize, tokenize};

fn main() {
    // let inputs = vec!["=1.2+10+\"abc\""; 100_000]
    //     .iter()
    //     .map(|f| f.to_string())
    //     .collect();

    // let start = std::time::Instant::now();

    // let tokens = parallel_tokenize(inputs);

    // println!("n: {:?}", tokens.len());
    // println!("time: {:?}", start.elapsed());
    println!("tokens: {:?}", tokenize("=#REF"));
}

// const tokenize = o_spreadsheet.tokenize;
// const inputs = [];
// for (let i = 0; i < 100000; i++) {
//     inputs.push("=10+\"abc\"");
// }
// const start = Date.now();
// const tokens = inputs.map((f) => tokenize(f));
// const time = Date.now() - start;
