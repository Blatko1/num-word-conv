#[path = "../benches/version1.rs"]
mod version1;
#[path = "../benches/version2.rs"]
mod version2;

use crate::{convert, Number as Num};
use std::io::Write;
use version1::num_word_conv as conv1;
use version2::num_word_conv as conv2;

#[test]
fn units() {
    let g = [Num::Zero, Num::Zero, Num::One];
    assert_eq!(convert(g, false).0, "jedan ");
    let g = [Num::Zero, Num::Zero, Num::Two];
    assert_eq!(convert(g, false).0, "dva ");
    let g = [Num::Zero, Num::Zero, Num::Three];
    assert_eq!(convert(g, false).0, "tri ");
    let g = [Num::Zero, Num::Zero, Num::Four];
    assert_eq!(convert(g, false).0, "četiri ");
    let g = [Num::Zero, Num::Zero, Num::Five];
    assert_eq!(convert(g, false).0, "pet ");
    let g = [Num::Zero, Num::Zero, Num::Six];
    assert_eq!(convert(g, false).0, "šest ");
    let g = [Num::Zero, Num::Zero, Num::Seven];
    assert_eq!(convert(g, false).0, "sedam ");
    let g = [Num::Zero, Num::Zero, Num::Eight];
    assert_eq!(convert(g, false).0, "osam ");
    let g = [Num::Zero, Num::Zero, Num::Nine];
    assert_eq!(convert(g, false).0, "devet ");
}

#[test]
fn tens() {
    let g = [Num::Zero, Num::One, Num::Zero];
    assert_eq!(convert(g, false).0, "deset ");
    let g = [Num::Zero, Num::Two, Num::Zero];
    assert_eq!(convert(g, false).0, "dvadeset ");
    let g = [Num::Zero, Num::Three, Num::Zero];
    assert_eq!(convert(g, false).0, "trideset ");
    let g = [Num::Zero, Num::Four, Num::Zero];
    assert_eq!(convert(g, false).0, "četrdeset ");
    let g = [Num::Zero, Num::Five, Num::Zero];
    assert_eq!(convert(g, false).0, "pedeset ");
    let g = [Num::Zero, Num::Six, Num::Zero];
    assert_eq!(convert(g, false).0, "šezdeset ");
    let g = [Num::Zero, Num::Seven, Num::Zero];
    assert_eq!(convert(g, false).0, "sedamdeset ");
    let g = [Num::Zero, Num::Eight, Num::Zero];
    assert_eq!(convert(g, false).0, "osamdeset ");
    let g = [Num::Zero, Num::Nine, Num::Zero];
    assert_eq!(convert(g, false).0, "devedeset ");
}

#[test]
fn naest() {
    let g = [Num::Zero, Num::One, Num::One];
    assert_eq!(convert(g, false).0, "jedanaest ");
    let g = [Num::Zero, Num::One, Num::Two];
    assert_eq!(convert(g, false).0, "dvanaest ");
    let g = [Num::Zero, Num::One, Num::Three];
    assert_eq!(convert(g, false).0, "trinaest ");
    let g = [Num::Zero, Num::One, Num::Four];
    assert_eq!(convert(g, false).0, "četrnaest ");
    let g = [Num::Zero, Num::One, Num::Five];
    assert_eq!(convert(g, false).0, "petnaest ");
    let g = [Num::Zero, Num::One, Num::Six];
    assert_eq!(convert(g, false).0, "šesnaest ");
    let g = [Num::Zero, Num::One, Num::Seven];
    assert_eq!(convert(g, false).0, "sedamnaest ");
    let g = [Num::Zero, Num::One, Num::Eight];
    assert_eq!(convert(g, false).0, "osamnaest ");
    let g = [Num::Zero, Num::One, Num::Nine];
    assert_eq!(convert(g, false).0, "devetnaest ");
}

#[test]
fn hundreds() {
    let g = [Num::One, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "sto ");
    let g = [Num::Two, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "dvjesto ");
    let g = [Num::Three, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "tristo ");
    let g = [Num::Four, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "četiristo ");
    let g = [Num::Five, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "petsto ");
    let g = [Num::Six, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "šesto ");
    let g = [Num::Seven, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "sedamsto ");
    let g = [Num::Eight, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "osamsto ");
    let g = [Num::Nine, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "devetsto ");
}

#[test]
fn special() {
    let g = [Num::Zero, Num::Zero, Num::Zero];
    assert_eq!(convert(g, false).0, "");
}

#[test]
fn compare_results() {
    let mut stdout = std::io::stdout().lock();
    for arg in 1..1_000_003 {
        let args = arg.to_string();

        let result1 = conv1(args.clone()).unwrap();
        let result2 = conv2(args).unwrap();

        assert_eq!(result1, result2);

        if arg % 10000 == 0 {
            writeln!(stdout, "_______NEXT______{}", arg).unwrap();
        }
    }
}
