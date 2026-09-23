//! 薄いバイナリ。ロジックは lib.rs にある。
//! ここにロジックを書くと、統合テストから呼べなくなる。

use ex047_lib_and_bin::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: ex047_lib_and_bin <path>");
        std::process::exit(2);
    };

    match run(path) {
        Ok(message) => println!("{message}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
