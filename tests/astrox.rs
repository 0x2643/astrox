// Private crates.
use astrox::astrox;

// Source hashes.
const INPUT_HASHES: [[u8; 32]; 10] = [
    [
        88, 101, 183, 41, 212, 156, 190, 48, 230, 97, 94, 105, 177, 86, 88, 84, 60, 239, 203, 124,
        63, 32, 160, 222, 34, 141, 50, 108, 138, 16, 90, 230,
    ],
    [
        131, 190, 160, 208, 86, 121, 144, 207, 78, 235, 188, 253, 251, 0, 62, 161, 72, 225, 55,
        184, 202, 212, 91, 125, 56, 204, 174, 214, 100, 28, 150, 15,
    ],
    [
        179, 131, 188, 255, 206, 72, 67, 215, 143, 44, 37, 210, 230, 54, 1, 114, 91, 104, 33, 126,
        162, 70, 130, 110, 176, 105, 75, 122, 212, 119, 158, 216,
    ],
    [
        39, 225, 250, 46, 101, 197, 139, 171, 106, 53, 239, 59, 245, 146, 31, 67, 140, 201, 190,
        86, 193, 24, 88, 104, 86, 69, 166, 231, 214, 240, 167, 63,
    ],
    [
        245, 133, 43, 254, 160, 213, 125, 60, 90, 42, 50, 123, 188, 209, 219, 125, 176, 250, 51,
        158, 201, 167, 122, 244, 98, 108, 163, 110, 66, 26, 110, 177,
    ],
    [
        209, 138, 166, 16, 5, 177, 37, 64, 133, 13, 211, 172, 189, 223, 107, 148, 194, 78, 132, 23,
        181, 25, 69, 149, 229, 95, 41, 73, 236, 217, 1, 119,
    ],
    [
        74, 36, 121, 180, 94, 215, 192, 195, 231, 0, 113, 44, 66, 171, 214, 173, 223, 29, 224, 201,
        26, 217, 139, 162, 153, 31, 15, 158, 221, 165, 97, 74,
    ],
    [
        49, 189, 89, 13, 141, 86, 53, 198, 164, 148, 121, 198, 40, 219, 66, 13, 251, 73, 38, 16,
        94, 71, 244, 44, 236, 119, 243, 202, 211, 199, 161, 164,
    ],
    [
        36, 23, 55, 103, 91, 244, 203, 137, 143, 244, 115, 22, 197, 152, 241, 34, 94, 40, 61, 246,
        64, 251, 232, 23, 91, 203, 48, 233, 13, 70, 19, 6,
    ],
    [
        19, 247, 78, 146, 5, 164, 136, 199, 248, 218, 200, 24, 110, 186, 2, 253, 192, 139, 161, 70,
        92, 156, 177, 77, 63, 124, 209, 236, 105, 130, 229, 218,
    ],
];

// Expected hashes.
const OUTPUT_HASHES: [[u8; 32]; 10] = [
    [
        212, 43, 99, 39, 66, 250, 87, 19, 104, 120, 109, 5, 140, 217, 104, 244, 130, 162, 100, 49,
        121, 154, 17, 140, 25, 63, 129, 55, 116, 200, 175, 147,
    ],
    [
        199, 166, 82, 73, 174, 32, 137, 247, 80, 190, 55, 41, 213, 254, 209, 60, 126, 136, 53, 115,
        15, 34, 162, 11, 34, 35, 68, 249, 192, 144, 0, 90,
    ],
    [
        112, 39, 223, 242, 147, 95, 235, 245, 191, 49, 222, 38, 157, 16, 130, 88, 222, 246, 94,
        191, 235, 85, 232, 185, 222, 165, 36, 209, 224, 194, 108, 200,
    ],
    [
        188, 48, 161, 21, 235, 210, 30, 33, 173, 131, 243, 62, 235, 6, 128, 237, 96, 246, 2, 185,
        232, 83, 136, 10, 189, 87, 24, 57, 249, 209, 28, 234,
    ],
    [
        26, 87, 37, 198, 10, 243, 120, 231, 6, 43, 195, 129, 36, 26, 48, 89, 47, 10, 52, 215, 255,
        16, 245, 214, 182, 16, 10, 83, 145, 178, 16, 36,
    ],
    [
        131, 144, 36, 10, 109, 62, 129, 156, 156, 27, 9, 78, 110, 10, 197, 229, 143, 154, 133, 200,
        195, 78, 205, 183, 127, 183, 53, 40, 28, 86, 38, 230,
    ],
    [
        72, 222, 124, 37, 30, 252, 207, 168, 155, 125, 47, 98, 102, 178, 43, 168, 27, 107, 31, 175,
        236, 237, 247, 101, 113, 94, 164, 169, 124, 36, 9, 53,
    ],
    [
        194, 90, 247, 105, 189, 118, 22, 21, 64, 69, 31, 105, 183, 97, 67, 184, 25, 243, 86, 155,
        141, 30, 224, 175, 28, 171, 85, 22, 193, 33, 178, 181,
    ],
    [
        153, 34, 94, 239, 183, 206, 96, 151, 59, 218, 171, 112, 146, 48, 165, 106, 221, 166, 99,
        175, 207, 72, 195, 217, 31, 185, 18, 121, 150, 203, 83, 192,
    ],
    [
        245, 32, 129, 79, 75, 160, 244, 194, 180, 83, 40, 93, 219, 56, 103, 199, 31, 61, 103, 156,
        47, 234, 35, 178, 198, 228, 197, 224, 81, 92, 30, 213,
    ],
];

// AstroX tests.
#[test]
fn astrox_hash_10() {
    for index in 0..INPUT_HASHES.len() {
        assert_eq!(
            OUTPUT_HASHES[index],
            astrox::astrox_hash(&INPUT_HASHES[index])
        );
    }
}
