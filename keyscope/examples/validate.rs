use std::env;

use keyscope;

fn main() {
    let github_key = env::args().nth(1).unwrap_or_default();

    let params = vec![github_key];
    let res = match keyscope::providers::GITHUB.key_validate(&params) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error occurred during validation: {err}");
            std::process::exit(1);
        }
    };
    println!("Key validation: {}", res.ok);
}
