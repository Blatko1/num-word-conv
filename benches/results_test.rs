mod version1;
mod version2;

use std::io::Write;
use version1::num_word_conv as conv1;
use version2::num_word_conv as conv2;

#[test]
fn compare_results() {
    let mut stdout = std::io::stdout().lock();
    for arg in 1..999_999_999 {
        let args = arg.to_string();

        let result1 = conv1(args.clone()).unwrap();
        let result2 = conv2(args).unwrap();

        assert_eq!(result1, result2);

        if arg % 10000 == 0 {
            writeln!(stdout, "_______NEXT______{}", arg).unwrap();
        }
    }
}
