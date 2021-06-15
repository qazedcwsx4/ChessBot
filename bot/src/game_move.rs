use serde_json::Number;
use std::ops::Sub;

const VALUE_A: i8 = 97;
const VALUE_1: i8 = 49;

struct Move {
    from: i8,
    to: i8,
}

impl From<(i8, i8)> for Move {
    fn from(tup: (i8, i8)) -> Move {
        Move {
            from: tup.0,
            to: tup.1,
        }
    }
}

impl From<((i8, i8), (i8, i8))> for Move {
    fn from(tup: ((i8, i8), (i8, i8))) -> Move {
        Move {
            from: tup.0.0 + tup.0.1 * 8,
            to: tup.1.0 + tup.1.1 * 8,
        }
    }
}

impl From<String> for Move {
    fn from(str: String) -> Move {
        let bytes = str.as_bytes();

        let from = decode_move(bytes[0], VALUE_A) + decode_move(bytes[1], VALUE_1) * 8;
        let to = decode_move(bytes[3], VALUE_A) + decode_move(bytes[4], VALUE_1) * 8;

        Move {
            from,
            to
        }
    }
}

fn decode_move(byte: u8, value: i8) -> i8 {
    i8::from_be_bytes([byte]) - value
}