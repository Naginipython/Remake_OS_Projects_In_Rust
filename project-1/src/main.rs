use std::{
    fs,
    process,
    env,
    collections::HashMap,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file w/filenames> \n", args[0]);
        process::exit(1);
    }

    let init_file = fs::read_to_string(&args[1])
        .expect("Error: File doesn't exist");
    let result: HashMap<String, Vec<usize>> = HashMap::new();

    let result: HashMap<String, Vec<usize>> = get_data(result, init_file);
    
    // Print results
    let output: String = create_output(result);
    println!("{output}");
}

fn get_data(mut result: HashMap<String, Vec<usize>>, init_file: String) -> HashMap<String, Vec<usize>>{
    for (num, line) in init_file.lines().enumerate() {
        let new_file: String = fs::read_to_string(line)
            .expect("Error: a line in the inputted is not a file");
        let words: Vec<&str> = new_file.split_whitespace().collect();

        for w in words {
            let new_w: String = w.chars().filter(|x| x.is_alphabetic()).collect();
            if !result.contains_key(&new_w) {
                result.insert(new_w, vec![num]);
            } else {
                result.get_mut(&new_w).unwrap().push(num);
            }
        }
    }
    result
}

fn create_output(result: HashMap<String, Vec<usize>>) -> String {
    let mut output = String::default();

    for (word, x) in result {
        output.push_str(&format!("\n{}: {:?}", &word, x));
    }

    output
}