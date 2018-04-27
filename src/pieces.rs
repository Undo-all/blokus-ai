use piece::*;
use shape::Shape;
use std::slice;

pub const PIECES: [Piece; 21] = [
    Piece {
        orientations: &[Shape {
            bits: 0x0000000000000001,
            attachments: &[0],
            width: 0,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000003,
                attachments: &[0, 1],
                width: 1,
            },
            Shape {
                bits: 0x0000000000004001,
                attachments: &[0, 14],
                width: 0,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000008003,
                attachments: &[0, 1, 15],
                width: 1,
            },
            Shape {
                bits: 0x0000000000004003,
                attachments: &[0, 1, 14],
                width: 1,
            },
            Shape {
                bits: 0x000000000000C002,
                attachments: &[1, 14, 15],
                width: 1,
            },
            Shape {
                bits: 0x000000000000C001,
                attachments: &[0, 14, 15],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000007,
                attachments: &[0, 2],
                width: 2,
            },
            Shape {
                bits: 0x0000000010004001,
                attachments: &[0, 28],
                width: 0,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x000000000000C003,
            attachments: &[0, 1, 14, 15],
            width: 1,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000001C002,
                attachments: &[1, 14, 16],
                width: 2,
            },
            Shape {
                bits: 0x0000000000008007,
                attachments: &[0, 2, 15],
                width: 2,
            },
            Shape {
                bits: 0x000000002000C002,
                attachments: &[1, 14, 29],
                width: 1,
            },
            Shape {
                bits: 0x000000001000C001,
                attachments: &[0, 15, 28],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000000F,
                attachments: &[0, 3],
                width: 3,
            },
            Shape {
                bits: 0x0000040010004001,
                attachments: &[0, 42],
                width: 0,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000001C004,
                attachments: &[2, 14, 16],
                width: 2,
            },
            Shape {
                bits: 0x000000000001C001,
                attachments: &[0, 14, 16],
                width: 2,
            },
            Shape {
                bits: 0x0000000000010007,
                attachments: &[0, 2, 16],
                width: 2,
            },
            Shape {
                bits: 0x0000000000004007,
                attachments: &[0, 2, 14],
                width: 2,
            },
            Shape {
                bits: 0x0000000030008002,
                attachments: &[1, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000030004001,
                attachments: &[0, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000020008003,
                attachments: &[0, 1, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000010004003,
                attachments: &[0, 1, 28],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000C006,
                attachments: &[1, 2, 14, 15],
                width: 2,
            },
            Shape {
                bits: 0x0000000000018003,
                attachments: &[0, 1, 15, 16],
                width: 2,
            },
            Shape {
                bits: 0x000000001000C002,
                attachments: &[1, 14, 15, 28],
                width: 1,
            },
            Shape {
                bits: 0x000000002000C001,
                attachments: &[0, 14, 15, 29],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000003C001,
                attachments: &[0, 14, 17],
                width: 3,
            },
            Shape {
                bits: 0x000000000003C008,
                attachments: &[3, 14, 17],
                width: 3,
            },
            Shape {
                bits: 0x000000000000400F,
                attachments: &[0, 3, 14],
                width: 3,
            },
            Shape {
                bits: 0x000000000002000F,
                attachments: &[0, 3, 17],
                width: 3,
            },
            Shape {
                bits: 0x0000080020008003,
                attachments: &[0, 1, 43],
                width: 1,
            },
            Shape {
                bits: 0x0000040010004003,
                attachments: &[0, 1, 42],
                width: 1,
            },
            Shape {
                bits: 0x00000C0020008002,
                attachments: &[1, 42, 43],
                width: 1,
            },
            Shape {
                bits: 0x00000C0010004001,
                attachments: &[0, 42, 43],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000070008002,
                attachments: &[1, 28, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000020008007,
                attachments: &[0, 2, 29],
                width: 2,
            },
            Shape {
                bits: 0x000000004001C004,
                attachments: &[2, 14, 30],
                width: 2,
            },
            Shape {
                bits: 0x000000001001C001,
                attachments: &[0, 16, 28],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000070004001,
                attachments: &[0, 28, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000070010004,
                attachments: &[2, 28, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000010004007,
                attachments: &[0, 2, 28],
                width: 2,
            },
            Shape {
                bits: 0x0000000040010007,
                attachments: &[0, 2, 30],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000C00E,
                attachments: &[1, 3, 14, 15],
                width: 3,
            },
            Shape {
                bits: 0x0000000000030007,
                attachments: &[0, 2, 16, 17],
                width: 3,
            },
            Shape {
                bits: 0x0000000000038003,
                attachments: &[0, 1, 15, 17],
                width: 3,
            },
            Shape {
                bits: 0x000000000001C00C,
                attachments: &[2, 3, 14, 16],
                width: 3,
            },
            Shape {
                bits: 0x000004001000C002,
                attachments: &[1, 14, 15, 42],
                width: 1,
            },
            Shape {
                bits: 0x000008002000C001,
                attachments: &[0, 14, 15, 43],
                width: 1,
            },
            Shape {
                bits: 0x0000080030004001,
                attachments: &[0, 28, 29, 43],
                width: 1,
            },
            Shape {
                bits: 0x0000040030008002,
                attachments: &[1, 28, 29, 42],
                width: 1,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000001001C004,
                attachments: &[2, 14, 16, 28],
                width: 2,
            },
            Shape {
                bits: 0x000000004001C001,
                attachments: &[0, 14, 16, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000030008006,
                attachments: &[1, 2, 28, 29],
                width: 2,
            },
            Shape {
                bits: 0x0000000060008003,
                attachments: &[0, 1, 29, 30],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000001F,
                attachments: &[0, 4],
                width: 4,
            },
            Shape {
                bits: 0x0100040010004001,
                attachments: &[0, 56],
                width: 0,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000003000C001,
                attachments: &[0, 15, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x000000003000C002,
                attachments: &[1, 14, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x000000001000C003,
                attachments: &[0, 1, 15, 28],
                width: 1,
            },
            Shape {
                bits: 0x000000002000C003,
                attachments: &[0, 1, 14, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000000018007,
                attachments: &[0, 2, 15, 16],
                width: 2,
            },
            Shape {
                bits: 0x000000000000C007,
                attachments: &[0, 2, 14, 15],
                width: 2,
            },
            Shape {
                bits: 0x000000000001C006,
                attachments: &[1, 2, 14, 16],
                width: 2,
            },
            Shape {
                bits: 0x000000000001C003,
                attachments: &[0, 1, 14, 16],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000001000C006,
                attachments: &[1, 2, 14, 15, 28],
                width: 2,
            },
            Shape {
                bits: 0x0000000040018003,
                attachments: &[0, 1, 15, 16, 30],
                width: 2,
            },
            Shape {
                bits: 0x000000006000C001,
                attachments: &[0, 14, 15, 29, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000030018004,
                attachments: &[2, 15, 16, 28, 29],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000030004003,
                attachments: &[0, 1, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000030008003,
                attachments: &[0, 1, 28, 29],
                width: 1,
            },
            Shape {
                bits: 0x0000000000014007,
                attachments: &[0, 2, 14, 16],
                width: 2,
            },
            Shape {
                bits: 0x000000000001C005,
                attachments: &[0, 2, 14, 16],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000002000C006,
                attachments: &[1, 2, 14, 29],
                width: 2,
            },
            Shape {
                bits: 0x0000000020018003,
                attachments: &[0, 1, 16, 29],
                width: 2,
            },
            Shape {
                bits: 0x000000006000C002,
                attachments: &[1, 14, 29, 30],
                width: 2,
            },
            Shape {
                bits: 0x0000000030018002,
                attachments: &[1, 16, 28, 29],
                width: 2,
            },
            Shape {
                bits: 0x000000001001C002,
                attachments: &[1, 14, 16, 28],
                width: 2,
            },
            Shape {
                bits: 0x000000004001C002,
                attachments: &[1, 14, 16, 30],
                width: 2,
            },
            Shape {
                bits: 0x000000002001C001,
                attachments: &[0, 14, 16, 29],
                width: 2,
            },
            Shape {
                bits: 0x000000002001C004,
                attachments: &[2, 14, 16, 29],
                width: 2,
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x000000002001C002,
            attachments: &[1, 14, 16, 29],
            width: 2,
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000003C002,
                attachments: &[1, 14, 17],
                width: 3,
            },
            Shape {
                bits: 0x000000000003C004,
                attachments: &[2, 14, 17],
                width: 3,
            },
            Shape {
                bits: 0x000000000000800F,
                attachments: &[0, 3, 15],
                width: 3,
            },
            Shape {
                bits: 0x000000000001000F,
                attachments: &[0, 3, 16],
                width: 3,
            },
            Shape {
                bits: 0x000008002000C002,
                attachments: &[1, 14, 43],
                width: 1,
            },
            Shape {
                bits: 0x000004001000C001,
                attachments: &[0, 15, 42],
                width: 1,
            },
            Shape {
                bits: 0x0000080030008002,
                attachments: &[1, 28, 43],
                width: 1,
            },
            Shape {
                bits: 0x0000040030004001,
                attachments: &[0, 29, 42],
                width: 1,
            },
        ],
    },
];

/*
// This was autogenerated. There will likely be more autogenerated code
// added here in the future for the sake of faster move generation.
const PIECES: [Piece; 21] = [
    Piece {
        orientations: &[Shape {
            bits: 0x0000000000000001,
            attachments: &[0],
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000003,
                attachments: &[0, 1],
            },
            Shape {
                bits: 0x0000000000004001,
                attachments: &[0, 14],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000008003,
                attachments: &[0, 1, 15],
            },
            Shape {
                bits: 0x0000000000004003,
                attachments: &[0, 1, 14],
            },
            Shape {
                bits: 0x000000000000C002,
                attachments: &[1, 14, 15],
            },
            Shape {
                bits: 0x000000000000C001,
                attachments: &[0, 14, 15],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000000000007,
                attachments: &[0, 2],
            },
            Shape {
                bits: 0x0000000010004001,
                attachments: &[0, 28],
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x000000000000C003,
            attachments: &[0, 1, 14, 15],
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000001C002,
                attachments: &[1, 14, 16],
            },
            Shape {
                bits: 0x0000000000008007,
                attachments: &[0, 2, 15],
            },
            Shape {
                bits: 0x000000002000C002,
                attachments: &[1, 14, 29],
            },
            Shape {
                bits: 0x000000001000C001,
                attachments: &[0, 15, 28],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000000F,
                attachments: &[0, 3],
            },
            Shape {
                bits: 0x0000040010004001,
                attachments: &[0, 42],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000001C004,
                attachments: &[2, 14, 16],
            },
            Shape {
                bits: 0x000000000001C001,
                attachments: &[0, 14, 16],
            },
            Shape {
                bits: 0x0000000000010007,
                attachments: &[0, 2, 16],
            },
            Shape {
                bits: 0x0000000000004007,
                attachments: &[0, 2, 14],
            },
            Shape {
                bits: 0x0000000030008002,
                attachments: &[1, 28, 29],
            },
            Shape {
                bits: 0x0000000030004001,
                attachments: &[0, 28, 29],
            },
            Shape {
                bits: 0x0000000020008003,
                attachments: &[0, 1, 29],
            },
            Shape {
                bits: 0x0000000010004003,
                attachments: &[0, 1, 28],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000C006,
                attachments: &[1, 2, 14, 15],
            },
            Shape {
                bits: 0x0000000000018003,
                attachments: &[0, 1, 15, 16],
            },
            Shape {
                bits: 0x000000001000C002,
                attachments: &[1, 14, 15, 28],
            },
            Shape {
                bits: 0x000000002000C001,
                attachments: &[0, 14, 15, 29],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000003C001,
                attachments: &[0, 14, 17],
            },
            Shape {
                bits: 0x000000000003C008,
                attachments: &[3, 14, 17],
            },
            Shape {
                bits: 0x000000000000400F,
                attachments: &[0, 3, 14],
            },
            Shape {
                bits: 0x000000000002000F,
                attachments: &[0, 3, 17],
            },
            Shape {
                bits: 0x0000080020008003,
                attachments: &[0, 1, 43],
            },
            Shape {
                bits: 0x0000040010004003,
                attachments: &[0, 1, 42],
            },
            Shape {
                bits: 0x00000C0020008002,
                attachments: &[1, 42, 43],
            },
            Shape {
                bits: 0x00000C0010004001,
                attachments: &[0, 42, 43],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000070008002,
                attachments: &[1, 28, 30],
            },
            Shape {
                bits: 0x0000000020008007,
                attachments: &[0, 2, 29],
            },
            Shape {
                bits: 0x000000004001C004,
                attachments: &[2, 14, 30],
            },
            Shape {
                bits: 0x000000001001C001,
                attachments: &[0, 16, 28],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000070004001,
                attachments: &[0, 28, 30],
            },
            Shape {
                bits: 0x0000000070010004,
                attachments: &[2, 28, 30],
            },
            Shape {
                bits: 0x0000000010004007,
                attachments: &[0, 2, 28],
            },
            Shape {
                bits: 0x0000000040010007,
                attachments: &[0, 2, 30],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000C00E,
                attachments: &[1, 3, 14, 15],
            },
            Shape {
                bits: 0x0000000000030007,
                attachments: &[0, 2, 16, 17],
            },
            Shape {
                bits: 0x0000000000038003,
                attachments: &[0, 1, 15, 17],
            },
            Shape {
                bits: 0x000000000001C00C,
                attachments: &[2, 3, 14, 16],
            },
            Shape {
                bits: 0x000004001000C002,
                attachments: &[1, 14, 15, 42],
            },
            Shape {
                bits: 0x000008002000C001,
                attachments: &[0, 14, 15, 43],
            },
            Shape {
                bits: 0x0000080030004001,
                attachments: &[0, 28, 29, 43],
            },
            Shape {
                bits: 0x0000040030008002,
                attachments: &[1, 28, 29, 42],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000001001C004,
                attachments: &[2, 14, 16, 28],
            },
            Shape {
                bits: 0x000000004001C001,
                attachments: &[0, 14, 16, 30],
            },
            Shape {
                bits: 0x0000000030008006,
                attachments: &[1, 2, 28, 29],
            },
            Shape {
                bits: 0x0000000060008003,
                attachments: &[0, 1, 29, 30],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000000001F,
                attachments: &[0, 4],
            },
            Shape {
                bits: 0x0100040010004001,
                attachments: &[0, 56],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000003000C001,
                attachments: &[0, 15, 28, 29],
            },
            Shape {
                bits: 0x000000003000C002,
                attachments: &[1, 14, 28, 29],
            },
            Shape {
                bits: 0x000000001000C003,
                attachments: &[0, 1, 15, 28],
            },
            Shape {
                bits: 0x000000002000C003,
                attachments: &[0, 1, 14, 29],
            },
            Shape {
                bits: 0x0000000000018007,
                attachments: &[0, 2, 15, 16],
            },
            Shape {
                bits: 0x000000000000C007,
                attachments: &[0, 2, 14, 15],
            },
            Shape {
                bits: 0x000000000001C006,
                attachments: &[1, 2, 14, 16],
            },
            Shape {
                bits: 0x000000000001C003,
                attachments: &[0, 1, 14, 16],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000001000C006,
                attachments: &[1, 2, 14, 15, 28],
            },
            Shape {
                bits: 0x0000000040018003,
                attachments: &[0, 1, 15, 16, 30],
            },
            Shape {
                bits: 0x000000006000C001,
                attachments: &[0, 14, 15, 29, 30],
            },
            Shape {
                bits: 0x0000000030018004,
                attachments: &[2, 15, 16, 28, 29],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x0000000030004003,
                attachments: &[0, 1, 28, 29],
            },
            Shape {
                bits: 0x0000000030008003,
                attachments: &[0, 1, 28, 29],
            },
            Shape {
                bits: 0x0000000000014007,
                attachments: &[0, 2, 14, 16],
            },
            Shape {
                bits: 0x000000000001C005,
                attachments: &[0, 2, 14, 16],
            },
        ],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000002000C006,
                attachments: &[1, 2, 14, 29],
            },
            Shape {
                bits: 0x0000000020018003,
                attachments: &[0, 1, 16, 29],
            },
            Shape {
                bits: 0x000000006000C002,
                attachments: &[1, 14, 29, 30],
            },
            Shape {
                bits: 0x0000000030018002,
                attachments: &[1, 16, 28, 29],
            },
            Shape {
                bits: 0x000000001001C002,
                attachments: &[1, 14, 16, 28],
            },
            Shape {
                bits: 0x000000004001C002,
                attachments: &[1, 14, 16, 30],
            },
            Shape {
                bits: 0x000000002001C001,
                attachments: &[0, 14, 16, 29],
            },
            Shape {
                bits: 0x000000002001C004,
                attachments: &[2, 14, 16, 29],
            },
        ],
    },
    Piece {
        orientations: &[Shape {
            bits: 0x000000002001C002,
            attachments: &[1, 14, 16, 29],
        }],
    },
    Piece {
        orientations: &[
            Shape {
                bits: 0x000000000003C002,
                attachments: &[1, 14, 17],
            },
            Shape {
                bits: 0x000000000003C004,
                attachments: &[2, 14, 17],
            },
            Shape {
                bits: 0x000000000000800F,
                attachments: &[0, 3, 15],
            },
            Shape {
                bits: 0x000000000001000F,
                attachments: &[0, 3, 16],
            },
            Shape {
                bits: 0x000008002000C002,
                attachments: &[1, 14, 43],
            },
            Shape {
                bits: 0x000004001000C001,
                attachments: &[0, 15, 42],
            },
            Shape {
                bits: 0x0000080030008002,
                attachments: &[1, 28, 43],
            },
            Shape {
                bits: 0x0000040030004001,
                attachments: &[0, 29, 42],
            },
        ],
    },
];
*/

pub fn iter() -> slice::Iter<'static, Piece> {
    PIECES.iter()
}

pub fn by_id(id: PieceId) -> &'static Piece {
    unsafe { PIECES.get_unchecked(id as usize) }
}

