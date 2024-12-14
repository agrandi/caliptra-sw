/*++

Licensed under the Apache-2.0 license.

File Name:

    ml_dsa87_tests.rs

Abstract:

    File contains test cases for ML_DSA87 API tests

--*/

#![no_std]
#![no_main]

use caliptra_cfi_lib::CfiCounter;
use caliptra_drivers::{
    Array4x12, Hmac, HmacData, HmacKey, HmacMode, HmacTag, KeyId, KeyReadArgs, KeyUsage,
    KeyWriteArgs, Mldsa87, Mldsa87Msg, Mldsa87PubKey, Mldsa87Result, Mldsa87SignRnd,
    Mldsa87Signature, Trng,
};
use caliptra_registers::csrng::CsrngReg;
use caliptra_registers::entropy_src::EntropySrcReg;
use caliptra_registers::hmac::HmacReg;
use caliptra_registers::mldsa::MldsaReg;
use caliptra_registers::soc_ifc::SocIfcReg;
use caliptra_registers::soc_ifc_trng::SocIfcTrngReg;
use caliptra_test_harness::test_suite;

const PUBKEY: [u8; 2592] = [
    3, 150, 29, 157, 48, 42, 75, 210, 178, 196, 237, 151, 124, 210, 172, 220, 249, 189, 35, 58, 18,
    100, 160, 211, 251, 233, 195, 1, 144, 192, 226, 244, 91, 121, 82, 155, 217, 208, 41, 223, 176,
    224, 57, 48, 173, 127, 130, 88, 110, 49, 240, 93, 249, 186, 238, 46, 84, 80, 51, 189, 239, 242,
    249, 11, 28, 106, 86, 190, 50, 98, 92, 205, 217, 43, 193, 113, 25, 158, 165, 78, 2, 53, 189,
    136, 146, 67, 110, 103, 53, 86, 72, 35, 60, 12, 240, 46, 243, 43, 133, 40, 29, 227, 236, 158,
    119, 47, 124, 81, 191, 129, 111, 220, 51, 62, 250, 38, 124, 148, 59, 145, 183, 194, 15, 192,
    47, 18, 228, 8, 4, 187, 122, 114, 138, 136, 116, 208, 34, 75, 9, 238, 61, 255, 216, 69, 228,
    60, 231, 58, 84, 143, 27, 165, 237, 110, 168, 14, 30, 252, 247, 144, 174, 68, 145, 255, 203,
    210, 73, 210, 149, 32, 6, 104, 51, 128, 70, 220, 156, 144, 242, 227, 192, 91, 6, 201, 161, 68,
    178, 227, 89, 202, 38, 185, 211, 237, 234, 43, 172, 137, 245, 119, 227, 151, 65, 153, 15, 23,
    192, 193, 131, 91, 152, 123, 179, 24, 36, 140, 216, 133, 193, 92, 118, 35, 240, 28, 218, 194,
    185, 138, 109, 215, 193, 191, 167, 102, 56, 57, 96, 164, 37, 140, 59, 199, 133, 12, 239, 21,
    56, 8, 206, 100, 5, 190, 234, 74, 127, 90, 226, 157, 84, 230, 142, 202, 147, 196, 77, 118, 83,
    224, 46, 227, 250, 242, 52, 161, 156, 218, 37, 81, 254, 78, 107, 140, 165, 151, 205, 190, 91,
    232, 29, 183, 159, 132, 153, 221, 196, 158, 191, 59, 254, 85, 35, 109, 153, 54, 152, 90, 102,
    150, 193, 244, 3, 26, 16, 192, 50, 145, 194, 9, 98, 107, 72, 81, 252, 42, 37, 64, 251, 170,
    119, 144, 255, 237, 34, 134, 120, 68, 245, 6, 194, 4, 217, 21, 253, 61, 47, 154, 105, 205, 160,
    58, 191, 26, 51, 83, 188, 114, 40, 206, 91, 5, 209, 176, 97, 206, 73, 121, 109, 182, 81, 95,
    52, 113, 156, 29, 77, 12, 219, 76, 168, 121, 178, 194, 158, 140, 209, 94, 126, 100, 7, 151,
    197, 56, 120, 53, 41, 92, 128, 30, 13, 80, 216, 255, 78, 180, 30, 206, 186, 8, 214, 34, 87,
    122, 55, 144, 85, 97, 78, 224, 11, 168, 88, 23, 100, 223, 118, 252, 126, 206, 93, 89, 98, 138,
    107, 55, 154, 199, 214, 11, 36, 24, 148, 120, 173, 133, 177, 81, 172, 155, 82, 126, 108, 187,
    123, 142, 191, 103, 39, 192, 226, 210, 180, 113, 136, 179, 13, 139, 56, 10, 134, 61, 163, 161,
    102, 81, 41, 6, 164, 77, 16, 226, 57, 88, 15, 21, 41, 246, 135, 20, 54, 254, 36, 223, 4, 70,
    142, 71, 9, 11, 203, 254, 247, 15, 57, 183, 166, 98, 34, 81, 199, 92, 79, 100, 147, 146, 61,
    12, 178, 96, 107, 181, 243, 76, 17, 209, 132, 245, 122, 70, 99, 2, 100, 80, 6, 63, 249, 91, 36,
    136, 58, 203, 168, 197, 107, 151, 198, 86, 79, 117, 238, 210, 133, 208, 76, 105, 108, 86, 122,
    174, 163, 196, 18, 175, 11, 248, 252, 188, 130, 117, 230, 180, 194, 114, 116, 139, 120, 237, 6,
    64, 231, 173, 55, 233, 165, 230, 69, 7, 211, 37, 101, 141, 101, 19, 235, 113, 213, 49, 230, 15,
    8, 33, 117, 165, 236, 34, 131, 167, 207, 198, 194, 17, 85, 84, 8, 186, 172, 254, 93, 181, 157,
    106, 34, 36, 19, 143, 147, 199, 156, 207, 9, 2, 115, 4, 126, 131, 64, 110, 228, 27, 97, 208,
    127, 250, 143, 5, 153, 215, 17, 100, 36, 69, 156, 209, 238, 245, 160, 33, 228, 198, 109, 84, 1,
    8, 14, 89, 178, 154, 101, 42, 80, 55, 72, 115, 201, 68, 226, 159, 36, 168, 178, 166, 59, 203,
    64, 7, 248, 160, 110, 176, 236, 201, 93, 71, 129, 206, 150, 244, 83, 32, 5, 117, 74, 89, 111,
    104, 113, 22, 15, 15, 215, 225, 246, 152, 137, 55, 125, 71, 221, 79, 151, 57, 102, 109, 61,
    178, 127, 3, 48, 144, 209, 58, 254, 112, 104, 4, 109, 187, 121, 52, 63, 224, 67, 177, 131, 192,
    129, 95, 64, 149, 140, 142, 102, 195, 133, 111, 70, 194, 164, 81, 185, 85, 216, 147, 146, 250,
    61, 158, 11, 114, 91, 38, 45, 252, 119, 93, 56, 59, 154, 87, 90, 144, 73, 180, 179, 173, 56,
    90, 153, 69, 14, 28, 49, 202, 94, 7, 227, 64, 222, 242, 254, 45, 192, 122, 123, 66, 116, 246,
    64, 98, 151, 242, 124, 31, 180, 205, 177, 118, 195, 60, 117, 17, 93, 81, 109, 128, 26, 20, 63,
    248, 131, 2, 113, 238, 159, 38, 99, 91, 158, 93, 86, 42, 9, 198, 233, 239, 66, 252, 141, 70,
    95, 19, 200, 153, 15, 166, 68, 19, 113, 51, 215, 50, 178, 48, 58, 24, 12, 72, 50, 29, 233, 144,
    204, 98, 83, 216, 55, 46, 216, 186, 138, 40, 37, 74, 178, 139, 201, 216, 228, 231, 27, 52, 44,
    146, 53, 19, 58, 149, 229, 97, 251, 29, 232, 147, 254, 149, 158, 84, 233, 233, 240, 16, 138,
    78, 212, 166, 116, 152, 165, 144, 150, 154, 5, 138, 203, 197, 251, 149, 192, 26, 217, 192, 117,
    20, 49, 216, 179, 69, 16, 235, 191, 139, 194, 107, 120, 90, 60, 116, 81, 127, 10, 95, 85, 162,
    220, 114, 7, 57, 171, 140, 151, 207, 79, 63, 173, 254, 163, 239, 204, 144, 18, 5, 17, 151, 232,
    122, 22, 31, 105, 237, 141, 140, 10, 152, 125, 230, 205, 240, 212, 253, 69, 146, 47, 18, 156,
    6, 235, 15, 199, 183, 131, 211, 84, 233, 84, 133, 130, 245, 35, 208, 0, 246, 89, 60, 88, 56,
    202, 244, 192, 144, 198, 49, 177, 5, 17, 183, 52, 192, 117, 58, 6, 252, 113, 217, 8, 224, 202,
    103, 214, 26, 154, 219, 164, 51, 27, 27, 223, 89, 251, 102, 5, 156, 212, 219, 215, 218, 142,
    167, 206, 70, 192, 33, 133, 40, 57, 43, 129, 142, 78, 77, 133, 8, 164, 0, 137, 76, 24, 77, 214,
    157, 187, 162, 142, 102, 37, 108, 164, 234, 64, 56, 55, 54, 51, 226, 184, 241, 224, 211, 65,
    206, 179, 114, 191, 54, 55, 206, 97, 198, 42, 76, 18, 58, 82, 198, 30, 27, 27, 185, 110, 135,
    164, 2, 239, 206, 2, 70, 174, 109, 221, 197, 170, 244, 133, 217, 28, 231, 72, 127, 167, 217,
    219, 97, 189, 134, 135, 47, 125, 24, 146, 201, 236, 198, 172, 73, 145, 149, 128, 196, 36, 141,
    146, 193, 210, 243, 232, 21, 157, 3, 190, 185, 216, 183, 49, 52, 54, 143, 112, 129, 44, 144,
    50, 164, 142, 206, 107, 161, 89, 93, 107, 95, 88, 37, 203, 125, 26, 70, 23, 35, 248, 121, 121,
    45, 230, 123, 157, 46, 135, 8, 140, 125, 190, 188, 181, 206, 180, 34, 164, 240, 250, 83, 253,
    30, 35, 123, 228, 23, 145, 72, 149, 79, 248, 94, 194, 236, 30, 17, 34, 240, 240, 61, 76, 204,
    28, 38, 157, 77, 153, 242, 173, 84, 47, 178, 129, 237, 200, 146, 88, 223, 173, 184, 42, 201,
    138, 47, 239, 45, 224, 203, 151, 34, 13, 3, 65, 243, 24, 244, 47, 138, 122, 121, 152, 102, 107,
    251, 132, 95, 163, 125, 23, 190, 169, 122, 29, 80, 142, 193, 101, 38, 7, 7, 94, 90, 88, 227,
    20, 148, 33, 11, 41, 172, 217, 79, 181, 46, 33, 102, 17, 153, 8, 65, 93, 20, 82, 187, 78, 126,
    92, 122, 24, 191, 191, 135, 17, 214, 224, 155, 6, 165, 102, 68, 34, 165, 95, 82, 144, 29, 61,
    96, 172, 4, 132, 177, 70, 243, 72, 171, 56, 23, 210, 231, 53, 99, 10, 145, 93, 98, 171, 3, 104,
    89, 61, 99, 126, 237, 243, 13, 139, 21, 7, 243, 151, 94, 83, 73, 18, 124, 217, 84, 101, 192,
    37, 147, 228, 64, 108, 4, 31, 174, 9, 18, 186, 53, 33, 13, 247, 75, 246, 163, 180, 52, 201,
    234, 35, 6, 168, 123, 240, 137, 126, 122, 90, 127, 138, 29, 150, 47, 11, 132, 59, 117, 221,
    104, 180, 27, 206, 17, 20, 29, 32, 95, 43, 210, 228, 153, 122, 164, 138, 41, 159, 42, 213, 35,
    81, 113, 98, 145, 198, 255, 38, 156, 73, 208, 29, 15, 82, 66, 52, 211, 241, 22, 156, 155, 194,
    92, 85, 145, 31, 53, 133, 16, 48, 124, 194, 243, 163, 234, 228, 107, 131, 18, 41, 250, 71, 253,
    97, 5, 114, 70, 48, 243, 78, 228, 66, 76, 162, 134, 108, 148, 174, 103, 245, 182, 51, 49, 57,
    98, 136, 205, 147, 249, 212, 223, 130, 204, 66, 154, 112, 148, 69, 121, 98, 153, 175, 0, 201,
    50, 145, 240, 148, 155, 58, 55, 155, 115, 148, 186, 18, 245, 130, 134, 218, 157, 90, 165, 53,
    218, 101, 238, 204, 141, 27, 143, 27, 229, 187, 88, 34, 215, 210, 141, 109, 194, 78, 253, 213,
    116, 71, 28, 157, 150, 242, 18, 166, 68, 218, 72, 227, 155, 43, 34, 100, 118, 71, 180, 211, 31,
    112, 63, 42, 186, 121, 220, 130, 77, 104, 125, 202, 231, 102, 100, 229, 12, 168, 247, 78, 151,
    251, 6, 36, 217, 88, 154, 149, 232, 244, 212, 56, 185, 249, 184, 135, 208, 93, 234, 176, 130,
    224, 182, 124, 205, 113, 142, 10, 54, 176, 112, 13, 13, 194, 219, 95, 160, 36, 171, 254, 132,
    152, 195, 32, 10, 147, 17, 188, 214, 39, 163, 204, 17, 87, 162, 203, 252, 242, 139, 86, 166,
    27, 164, 15, 56, 30, 117, 106, 2, 195, 177, 149, 196, 54, 255, 77, 145, 175, 192, 38, 39, 210,
    244, 143, 76, 78, 179, 196, 242, 172, 230, 89, 14, 189, 217, 189, 164, 138, 183, 4, 236, 223,
    176, 177, 133, 123, 163, 131, 51, 247, 164, 201, 56, 40, 210, 164, 157, 158, 212, 159, 62, 23,
    252, 182, 208, 26, 116, 46, 208, 224, 40, 48, 136, 145, 235, 235, 187, 144, 246, 59, 134, 227,
    138, 10, 157, 170, 79, 190, 223, 223, 227, 216, 7, 195, 71, 166, 48, 136, 132, 111, 177, 152,
    178, 66, 83, 2, 52, 173, 34, 115, 90, 123, 56, 93, 115, 202, 134, 191, 148, 88, 135, 7, 124,
    64, 112, 73, 34, 66, 84, 206, 216, 25, 138, 194, 39, 224, 117, 187, 60, 15, 89, 60, 37, 99,
    208, 18, 67, 244, 100, 130, 184, 167, 98, 12, 28, 132, 153, 71, 178, 32, 208, 0, 105, 198, 251,
    187, 50, 229, 165, 252, 1, 172, 23, 253, 55, 112, 84, 156, 171, 93, 225, 159, 10, 170, 65, 214,
    252, 4, 70, 92, 15, 242, 130, 215, 225, 230, 236, 198, 15, 238, 90, 91, 238, 154, 68, 248, 29,
    138, 92, 76, 64, 3, 139, 41, 167, 128, 23, 95, 55, 242, 79, 161, 200, 153, 225, 80, 20, 229,
    48, 204, 61, 124, 109, 3, 226, 224, 148, 204, 121, 243, 131, 186, 20, 192, 94, 111, 232, 234,
    128, 98, 2, 196, 247, 252, 33, 135, 44, 48, 42, 48, 201, 228, 149, 119, 135, 26, 106, 32, 184,
    156, 250, 193, 181, 148, 210, 25, 105, 253, 55, 148, 43, 64, 122, 174, 101, 236, 217, 43, 241,
    181, 204, 98, 89, 153, 253, 32, 214, 85, 190, 203, 75, 40, 47, 212, 245, 233, 60, 215, 211, 48,
    92, 133, 184, 59, 33, 217, 185, 12, 60, 223, 68, 120, 69, 135, 180, 118, 102, 143, 95, 187, 3,
    205, 189, 131, 86, 63, 140, 49, 97, 150, 13, 90, 179, 189, 4, 51, 75, 64, 55, 90, 116, 126,
    108, 249, 254, 93, 245, 21, 26, 253, 143, 56, 55, 136, 100, 90, 60, 113, 108, 247, 244, 219,
    68, 78, 194, 241, 140, 79, 3, 182, 138, 255, 86, 35, 157, 237, 203, 185, 44, 59, 50, 254, 215,
    229, 223, 229, 106, 19, 45, 246, 195, 225, 5, 155, 3, 230, 37, 120, 91, 78, 237, 121, 64, 184,
    73, 219, 38, 252, 1, 137, 203, 125, 168, 17, 173, 115, 132, 207, 235, 205, 145, 153, 67, 251,
    162, 136, 185, 109, 112, 116, 51, 229, 93, 24, 32, 153, 253, 71, 227, 169, 212, 105, 154, 103,
    105, 110, 153, 83, 247, 116, 192, 131, 149, 8, 159, 104, 225, 162, 105, 9, 100, 219, 240, 188,
    173, 66, 58, 170, 196, 189, 235, 42, 31, 78, 105, 94, 72, 3, 31, 189, 159, 58, 203, 35, 228,
    41, 96, 53, 24, 230, 207, 104, 27, 32, 229, 196, 101, 23, 251, 91, 70, 110, 19, 51, 182, 249,
    49, 17, 238, 114, 156, 202, 10, 139, 187, 18, 31, 181, 164, 48, 98, 54, 211, 108, 143, 125,
    156, 101, 122, 178, 81, 20, 247, 65, 76, 221, 106, 199, 71, 57, 156, 145, 107, 62, 162, 52, 2,
    154, 59, 199, 71, 118, 159, 35, 153, 194, 83, 221, 101, 115, 149, 162, 142, 189, 102, 246, 174,
    78, 254, 185, 81, 147, 216, 90, 75, 19, 195, 201, 190, 110, 74, 235, 203, 117, 101, 182, 95,
    51, 162, 25, 14, 63, 165, 185, 81, 96, 16, 79, 88, 23, 234, 136, 96, 9, 238, 245, 67, 107, 195,
    90, 152, 14, 66, 26, 98, 54, 57, 203, 112, 5, 171, 220, 155, 57, 42, 66, 225, 252, 70, 199,
    196, 80, 251, 58, 17, 125, 111, 71, 195, 249, 64, 74, 167, 126, 117, 155, 177, 130, 38, 37, 76,
    104, 198, 167, 130, 21, 118, 45, 83, 70, 111, 232, 1, 146, 41, 98, 51, 36, 221, 92, 223, 108,
    214, 239, 30, 155, 61, 47, 216, 93, 169, 58, 67, 150, 76, 192, 186, 36, 58, 106, 82, 65, 112,
    237, 176, 198, 22, 181, 152, 71, 152, 72, 225, 98, 96, 10, 250, 108, 114, 138, 248, 250, 82,
    19, 56, 48, 85, 203, 100, 92, 255, 0, 213, 29, 120, 6, 181, 40, 168, 169, 222, 58, 48, 111, 29,
    57, 193, 108, 34, 24, 19, 124, 222, 145, 33, 128, 29, 241, 154, 117, 94, 125, 107, 220, 9, 62,
    173, 44, 224, 207, 185, 171, 134, 141, 190, 212, 173, 204, 164, 61, 166, 230, 126, 62, 159,
    177, 142, 241, 186, 78, 144, 57, 9, 58, 156, 99, 151, 160, 150, 157, 214, 130, 147, 129, 22,
    49, 233, 163, 238, 125, 109, 84, 209, 245, 198, 32, 212, 10, 7, 10, 152, 138, 201, 40, 8, 255,
    34, 194, 49, 78, 173, 114, 214, 200, 28, 7, 120, 194, 237, 241, 95, 104, 243, 30, 63, 213, 52,
    65, 154, 152, 52, 164, 150, 186, 207, 163, 1, 10, 151, 8, 84, 179, 105, 1, 15, 183, 5, 23, 21,
    165, 155, 103, 187, 84, 66, 131, 251, 238, 29, 21, 120, 68, 207, 251, 25, 203, 149, 198, 166,
    176, 155, 119, 42, 254, 120, 242, 31, 43, 214, 120, 71, 206, 83, 4, 142, 54, 106, 23, 165, 101,
    235, 83, 122, 221, 100, 7, 12, 61, 168,
];

const SIGNATURE: [u8; 4628] = [
    44, 163, 134, 8, 175, 100, 32, 226, 37, 211, 242, 103, 165, 67, 156, 114, 252, 141, 252, 112,
    136, 62, 82, 144, 194, 70, 221, 212, 106, 9, 135, 165, 141, 147, 157, 23, 237, 225, 245, 253,
    243, 244, 5, 208, 8, 198, 17, 243, 50, 21, 133, 110, 159, 3, 54, 118, 6, 38, 111, 137, 24, 77,
    147, 30, 26, 94, 167, 113, 133, 120, 77, 119, 125, 202, 196, 67, 110, 59, 62, 66, 129, 38, 38,
    35, 19, 28, 146, 143, 128, 98, 123, 210, 222, 148, 187, 49, 2, 59, 245, 167, 77, 131, 199, 212,
    214, 236, 114, 251, 181, 175, 203, 171, 109, 238, 223, 207, 100, 238, 63, 239, 203, 100, 99,
    201, 0, 38, 242, 220, 73, 176, 93, 137, 84, 27, 184, 101, 138, 143, 242, 145, 72, 226, 6, 216,
    67, 117, 32, 3, 31, 185, 138, 57, 15, 62, 222, 20, 146, 137, 117, 130, 210, 196, 166, 102, 34,
    44, 162, 31, 225, 124, 84, 234, 228, 156, 211, 210, 249, 31, 198, 179, 27, 29, 74, 60, 160,
    194, 92, 178, 7, 127, 114, 33, 38, 35, 89, 55, 21, 182, 248, 223, 40, 4, 224, 209, 6, 171, 45,
    197, 135, 125, 209, 89, 183, 158, 12, 201, 70, 86, 165, 94, 25, 48, 217, 218, 198, 225, 182,
    175, 105, 100, 167, 138, 138, 177, 226, 159, 221, 33, 139, 135, 163, 213, 153, 147, 128, 187,
    211, 194, 144, 70, 9, 111, 161, 32, 234, 29, 195, 28, 62, 172, 63, 50, 213, 12, 206, 97, 66,
    163, 11, 66, 227, 190, 109, 149, 98, 102, 123, 210, 31, 185, 191, 121, 30, 93, 184, 230, 48,
    222, 28, 129, 5, 86, 99, 186, 88, 241, 229, 231, 5, 25, 209, 230, 48, 94, 209, 234, 134, 54,
    103, 46, 211, 248, 244, 4, 177, 54, 132, 199, 173, 80, 110, 157, 38, 171, 196, 242, 97, 150, 2,
    78, 145, 8, 174, 21, 175, 183, 175, 48, 11, 151, 224, 231, 239, 53, 164, 76, 19, 156, 110, 102,
    162, 102, 18, 166, 196, 183, 157, 251, 21, 214, 243, 207, 149, 170, 109, 107, 197, 134, 124,
    59, 215, 38, 48, 98, 23, 14, 111, 118, 185, 8, 46, 225, 229, 37, 223, 206, 177, 23, 14, 172,
    54, 33, 192, 34, 68, 182, 72, 110, 184, 211, 211, 49, 92, 54, 113, 118, 103, 38, 139, 15, 204,
    18, 91, 75, 37, 27, 27, 183, 48, 199, 248, 158, 205, 218, 248, 21, 163, 190, 133, 171, 233,
    224, 112, 130, 183, 112, 227, 123, 33, 245, 94, 50, 146, 166, 147, 254, 91, 4, 125, 52, 6, 253,
    124, 24, 253, 62, 146, 61, 172, 241, 24, 49, 88, 108, 234, 146, 211, 17, 10, 108, 19, 243, 220,
    82, 158, 172, 40, 86, 58, 109, 85, 193, 96, 161, 203, 121, 137, 86, 152, 248, 225, 70, 227, 27,
    94, 159, 44, 153, 76, 228, 108, 162, 210, 29, 133, 122, 28, 157, 117, 27, 202, 83, 101, 45, 63,
    238, 136, 40, 152, 230, 224, 40, 182, 67, 236, 167, 68, 137, 63, 222, 23, 27, 56, 159, 101,
    126, 189, 151, 103, 63, 70, 96, 147, 148, 21, 145, 125, 169, 244, 226, 91, 102, 251, 201, 239,
    63, 93, 217, 135, 197, 152, 172, 137, 194, 94, 127, 168, 122, 51, 187, 244, 192, 213, 250, 97,
    77, 62, 241, 75, 22, 122, 194, 4, 135, 193, 36, 213, 115, 41, 52, 43, 100, 184, 182, 132, 205,
    210, 107, 185, 48, 143, 198, 26, 124, 175, 18, 194, 196, 166, 220, 102, 192, 111, 141, 102,
    242, 248, 118, 226, 19, 232, 99, 14, 23, 254, 255, 249, 31, 36, 88, 86, 113, 16, 153, 103, 43,
    73, 26, 52, 224, 206, 147, 71, 177, 144, 109, 200, 255, 87, 100, 115, 145, 113, 187, 128, 75,
    176, 185, 78, 189, 253, 136, 170, 223, 82, 214, 7, 145, 179, 31, 75, 12, 202, 238, 115, 254,
    147, 9, 18, 50, 35, 139, 3, 118, 140, 50, 95, 14, 214, 14, 66, 144, 66, 22, 222, 34, 177, 210,
    20, 239, 242, 19, 237, 151, 249, 228, 120, 143, 97, 244, 59, 49, 160, 47, 20, 159, 170, 146,
    178, 120, 116, 230, 74, 119, 10, 66, 100, 212, 205, 177, 81, 181, 122, 119, 16, 7, 228, 134,
    108, 244, 166, 204, 139, 148, 40, 8, 244, 203, 48, 179, 195, 66, 11, 117, 161, 145, 217, 200,
    203, 48, 0, 67, 35, 252, 101, 47, 138, 69, 34, 127, 6, 8, 224, 48, 24, 219, 16, 215, 92, 201,
    226, 224, 100, 92, 239, 175, 21, 128, 169, 149, 79, 12, 133, 110, 135, 219, 228, 196, 100, 158,
    59, 151, 224, 98, 142, 140, 146, 195, 156, 245, 137, 118, 85, 173, 77, 95, 129, 150, 16, 238,
    191, 94, 48, 111, 200, 27, 55, 97, 225, 23, 230, 191, 188, 15, 182, 159, 207, 85, 168, 130, 9,
    43, 1, 222, 84, 225, 141, 182, 46, 174, 129, 136, 19, 16, 42, 184, 194, 48, 163, 8, 9, 46, 137,
    193, 149, 46, 34, 119, 0, 196, 45, 89, 63, 117, 116, 177, 75, 0, 50, 160, 82, 191, 119, 252,
    222, 199, 151, 223, 84, 61, 53, 89, 214, 109, 226, 117, 132, 194, 219, 24, 55, 141, 215, 202,
    162, 230, 178, 34, 175, 54, 92, 225, 252, 157, 111, 168, 153, 48, 194, 168, 230, 252, 213, 26,
    219, 105, 38, 21, 199, 22, 146, 176, 230, 61, 43, 214, 40, 178, 78, 53, 85, 49, 64, 153, 235,
    148, 192, 145, 215, 220, 46, 187, 138, 224, 223, 210, 23, 39, 244, 39, 138, 232, 176, 106, 176,
    102, 195, 93, 169, 2, 193, 235, 255, 111, 109, 228, 172, 14, 36, 31, 83, 82, 197, 73, 252, 209,
    28, 142, 160, 75, 134, 113, 104, 140, 40, 10, 98, 144, 32, 230, 51, 61, 209, 94, 221, 150, 149,
    136, 82, 3, 237, 53, 246, 126, 97, 45, 63, 151, 96, 168, 98, 226, 161, 211, 77, 25, 29, 11,
    109, 5, 249, 236, 112, 211, 250, 180, 83, 36, 229, 93, 239, 109, 40, 217, 123, 88, 237, 226,
    98, 1, 88, 210, 135, 126, 241, 110, 33, 77, 196, 217, 13, 240, 97, 250, 47, 85, 161, 241, 166,
    181, 34, 78, 133, 30, 93, 205, 1, 149, 230, 226, 84, 191, 132, 245, 87, 41, 11, 201, 157, 12,
    39, 132, 241, 21, 60, 191, 87, 250, 207, 177, 48, 237, 47, 83, 187, 121, 22, 233, 35, 134, 246,
    223, 150, 41, 71, 1, 148, 7, 118, 204, 203, 45, 110, 2, 66, 143, 118, 187, 138, 0, 142, 242,
    123, 101, 138, 72, 196, 164, 124, 4, 134, 90, 24, 226, 146, 144, 205, 65, 110, 124, 171, 236,
    177, 90, 54, 97, 149, 219, 113, 236, 119, 250, 192, 250, 1, 174, 95, 226, 171, 59, 84, 185,
    252, 195, 2, 178, 27, 45, 101, 176, 179, 210, 137, 136, 49, 28, 96, 8, 209, 3, 15, 7, 121, 27,
    104, 157, 112, 71, 176, 190, 238, 91, 156, 241, 153, 244, 87, 188, 55, 127, 186, 31, 1, 133,
    89, 123, 8, 96, 29, 84, 169, 204, 177, 11, 43, 121, 140, 175, 109, 187, 95, 199, 244, 26, 19,
    50, 11, 253, 53, 83, 223, 218, 130, 226, 100, 79, 33, 174, 152, 237, 38, 198, 244, 165, 163,
    221, 90, 50, 234, 246, 53, 129, 129, 0, 247, 207, 18, 113, 217, 64, 80, 18, 88, 208, 144, 241,
    80, 16, 81, 78, 87, 50, 213, 228, 203, 190, 228, 81, 145, 5, 60, 84, 164, 142, 14, 183, 154,
    71, 245, 98, 28, 41, 211, 157, 16, 220, 111, 36, 108, 211, 67, 5, 143, 44, 49, 4, 116, 29, 185,
    8, 198, 73, 250, 208, 30, 242, 7, 185, 160, 102, 80, 29, 192, 234, 92, 136, 206, 34, 212, 119,
    221, 30, 121, 124, 224, 52, 205, 130, 107, 66, 204, 145, 11, 255, 228, 134, 98, 82, 117, 199,
    113, 236, 10, 147, 5, 86, 47, 219, 205, 168, 238, 253, 43, 152, 70, 203, 28, 66, 20, 37, 66,
    84, 1, 86, 161, 214, 249, 121, 208, 218, 177, 24, 161, 215, 165, 226, 113, 6, 60, 182, 204,
    183, 130, 152, 75, 191, 160, 104, 249, 127, 15, 126, 26, 55, 72, 61, 109, 61, 149, 125, 109,
    73, 96, 81, 16, 123, 11, 4, 93, 196, 222, 175, 189, 207, 121, 139, 3, 201, 204, 79, 177, 238,
    77, 212, 38, 64, 85, 169, 10, 23, 156, 133, 220, 110, 145, 101, 49, 102, 174, 176, 158, 60,
    172, 35, 172, 101, 112, 63, 221, 32, 32, 186, 96, 52, 2, 167, 22, 142, 173, 9, 235, 96, 21, 88,
    82, 245, 242, 28, 39, 21, 193, 102, 30, 188, 17, 187, 65, 90, 8, 48, 8, 72, 184, 212, 142, 50,
    132, 212, 109, 147, 76, 69, 240, 244, 239, 147, 147, 62, 172, 159, 90, 75, 50, 177, 167, 118,
    183, 70, 102, 41, 119, 143, 1, 243, 163, 142, 228, 85, 124, 83, 232, 20, 190, 145, 217, 228,
    112, 65, 180, 179, 44, 236, 98, 5, 198, 208, 180, 123, 137, 110, 163, 95, 163, 72, 131, 41,
    214, 247, 79, 139, 130, 145, 237, 178, 230, 254, 112, 100, 193, 58, 36, 47, 241, 82, 192, 81,
    150, 13, 186, 185, 139, 202, 207, 202, 234, 12, 105, 169, 236, 208, 10, 190, 217, 110, 253,
    252, 142, 180, 251, 6, 63, 24, 173, 97, 59, 37, 189, 67, 157, 72, 219, 43, 96, 217, 88, 179,
    96, 169, 17, 56, 114, 138, 115, 147, 57, 187, 138, 165, 86, 74, 214, 136, 59, 8, 73, 139, 253,
    245, 21, 102, 3, 131, 63, 131, 102, 172, 112, 39, 40, 6, 240, 154, 133, 96, 20, 71, 199, 58,
    114, 185, 90, 226, 154, 159, 235, 189, 181, 110, 203, 23, 247, 22, 217, 48, 168, 45, 208, 33,
    65, 177, 202, 240, 191, 212, 223, 100, 200, 149, 28, 177, 13, 242, 88, 209, 153, 91, 56, 186,
    87, 77, 100, 162, 116, 237, 88, 2, 70, 163, 254, 190, 204, 55, 22, 1, 24, 71, 138, 37, 178, 25,
    232, 3, 47, 63, 74, 19, 141, 71, 31, 153, 55, 5, 247, 214, 177, 166, 98, 10, 107, 169, 139,
    245, 231, 180, 61, 12, 125, 34, 212, 165, 29, 225, 60, 96, 166, 223, 148, 201, 44, 33, 100, 72,
    150, 96, 253, 201, 29, 12, 155, 92, 177, 153, 175, 163, 26, 136, 77, 108, 131, 48, 132, 59, 98,
    248, 199, 200, 68, 99, 87, 77, 160, 62, 66, 51, 179, 121, 2, 207, 227, 102, 198, 186, 34, 126,
    211, 196, 112, 69, 193, 105, 58, 52, 23, 53, 182, 103, 243, 139, 213, 7, 93, 37, 57, 142, 180,
    108, 60, 62, 171, 135, 81, 168, 77, 95, 22, 135, 27, 124, 102, 51, 70, 15, 221, 116, 255, 158,
    123, 164, 77, 172, 1, 93, 38, 20, 212, 49, 103, 82, 177, 71, 198, 119, 4, 137, 38, 142, 34, 61,
    197, 196, 8, 159, 170, 216, 162, 50, 59, 193, 216, 67, 252, 100, 206, 237, 216, 64, 34, 178,
    139, 171, 172, 186, 143, 253, 22, 225, 11, 54, 210, 165, 246, 117, 172, 112, 243, 173, 80, 68,
    73, 254, 56, 209, 92, 230, 234, 117, 140, 155, 248, 155, 0, 227, 26, 222, 31, 188, 138, 215,
    164, 49, 254, 165, 239, 45, 137, 105, 9, 47, 146, 98, 90, 77, 194, 107, 247, 8, 140, 212, 215,
    63, 64, 26, 147, 246, 200, 65, 243, 120, 96, 70, 81, 186, 216, 226, 55, 57, 44, 36, 57, 186, 4,
    210, 122, 49, 106, 132, 209, 39, 95, 209, 52, 48, 22, 214, 93, 49, 47, 75, 127, 41, 251, 47,
    57, 216, 49, 161, 162, 45, 85, 196, 91, 132, 213, 111, 84, 83, 135, 14, 80, 142, 73, 24, 1, 88,
    70, 51, 215, 73, 188, 42, 11, 78, 138, 181, 217, 146, 178, 24, 43, 50, 48, 116, 247, 61, 84,
    18, 91, 151, 210, 238, 68, 54, 35, 187, 31, 153, 24, 249, 111, 246, 195, 189, 189, 243, 122,
    45, 235, 5, 69, 245, 101, 233, 123, 187, 202, 236, 246, 139, 238, 101, 95, 11, 205, 206, 112,
    114, 126, 67, 72, 32, 63, 62, 181, 251, 4, 156, 137, 170, 236, 18, 247, 64, 135, 171, 252, 156,
    75, 65, 147, 58, 245, 24, 12, 116, 154, 190, 5, 21, 95, 139, 223, 58, 2, 132, 231, 139, 123,
    232, 234, 205, 136, 166, 122, 27, 204, 202, 222, 55, 217, 21, 193, 149, 69, 226, 154, 127, 24,
    95, 61, 201, 249, 39, 18, 167, 69, 7, 244, 141, 55, 244, 117, 34, 151, 102, 121, 118, 1, 197,
    181, 250, 19, 54, 179, 30, 196, 185, 67, 33, 139, 58, 230, 13, 203, 132, 3, 26, 96, 212, 232,
    165, 232, 44, 159, 17, 22, 252, 6, 21, 213, 16, 95, 252, 50, 173, 41, 29, 72, 38, 169, 62, 32,
    220, 17, 242, 162, 186, 199, 24, 136, 22, 166, 249, 161, 11, 179, 224, 225, 95, 4, 32, 148,
    189, 199, 183, 150, 222, 231, 236, 14, 55, 58, 104, 17, 200, 63, 233, 31, 44, 9, 160, 95, 244,
    79, 176, 178, 181, 35, 64, 68, 10, 179, 108, 11, 142, 24, 230, 94, 27, 62, 163, 241, 57, 194,
    2, 242, 175, 187, 184, 73, 180, 83, 210, 238, 157, 69, 179, 51, 239, 91, 113, 201, 228, 133,
    131, 173, 194, 115, 49, 6, 39, 17, 134, 113, 76, 19, 229, 254, 1, 236, 198, 76, 92, 250, 167,
    64, 68, 85, 136, 186, 71, 224, 113, 57, 166, 161, 129, 165, 250, 201, 208, 16, 31, 9, 241, 6,
    7, 44, 197, 71, 118, 174, 190, 37, 206, 122, 9, 133, 100, 228, 87, 147, 57, 190, 24, 32, 253,
    4, 177, 245, 230, 171, 60, 90, 77, 11, 144, 136, 200, 119, 52, 249, 179, 253, 120, 86, 185, 29,
    77, 189, 194, 170, 240, 117, 91, 49, 195, 97, 199, 70, 133, 111, 187, 187, 17, 110, 68, 245,
    219, 226, 43, 174, 70, 57, 158, 130, 47, 43, 11, 229, 39, 128, 153, 226, 73, 218, 53, 93, 180,
    239, 250, 248, 74, 243, 212, 206, 185, 109, 161, 192, 194, 60, 61, 40, 1, 157, 158, 131, 58,
    154, 173, 203, 34, 220, 102, 237, 15, 249, 18, 246, 62, 18, 118, 43, 22, 54, 234, 110, 139, 74,
    138, 121, 24, 131, 237, 152, 157, 38, 183, 40, 125, 151, 182, 215, 33, 71, 138, 32, 128, 145,
    132, 159, 105, 75, 126, 132, 61, 31, 161, 3, 65, 44, 93, 96, 43, 88, 138, 70, 91, 20, 125, 31,
    130, 25, 157, 215, 224, 141, 131, 100, 107, 227, 8, 248, 53, 70, 10, 27, 84, 10, 139, 180, 111,
    198, 39, 152, 74, 170, 75, 30, 197, 235, 89, 252, 234, 133, 80, 209, 107, 125, 129, 215, 162,
    228, 179, 244, 78, 64, 211, 27, 104, 209, 109, 152, 232, 214, 252, 154, 253, 17, 115, 183, 214,
    1, 141, 110, 159, 130, 90, 72, 18, 99, 228, 241, 113, 194, 55, 115, 141, 107, 160, 229, 186,
    46, 58, 199, 97, 185, 161, 74, 91, 190, 148, 186, 31, 130, 77, 59, 184, 244, 138, 113, 42, 192,
    175, 79, 80, 31, 133, 72, 95, 100, 180, 13, 234, 39, 187, 243, 33, 48, 233, 61, 154, 42, 215,
    114, 156, 118, 179, 203, 170, 71, 135, 151, 134, 145, 142, 197, 124, 52, 50, 195, 45, 206, 182,
    249, 35, 51, 118, 54, 189, 67, 70, 68, 132, 221, 246, 5, 29, 187, 138, 32, 204, 99, 129, 94,
    68, 116, 66, 171, 120, 139, 95, 113, 185, 95, 81, 85, 99, 117, 90, 216, 188, 64, 195, 222, 126,
    215, 180, 58, 188, 51, 169, 173, 227, 215, 182, 250, 3, 119, 41, 131, 11, 53, 11, 108, 63, 215,
    91, 57, 70, 255, 83, 74, 168, 15, 42, 13, 46, 210, 156, 18, 18, 164, 84, 95, 169, 124, 4, 251,
    80, 33, 32, 97, 117, 255, 226, 136, 50, 155, 196, 68, 251, 126, 182, 71, 243, 243, 155, 53,
    204, 202, 36, 203, 109, 41, 162, 245, 198, 246, 198, 253, 230, 112, 103, 111, 12, 23, 66, 176,
    164, 45, 85, 55, 165, 101, 1, 116, 73, 188, 119, 186, 105, 74, 8, 87, 154, 104, 125, 122, 105,
    50, 124, 172, 78, 197, 110, 131, 6, 7, 43, 212, 225, 14, 12, 149, 117, 49, 164, 167, 128, 242,
    80, 68, 58, 182, 83, 161, 94, 175, 21, 119, 23, 157, 203, 247, 52, 54, 195, 205, 154, 191, 159,
    203, 32, 117, 99, 146, 8, 129, 84, 96, 108, 189, 45, 145, 193, 124, 76, 249, 7, 240, 19, 151,
    169, 242, 127, 11, 61, 108, 19, 141, 52, 26, 197, 127, 96, 27, 116, 69, 210, 111, 46, 161, 233,
    94, 48, 233, 37, 188, 90, 27, 214, 7, 123, 144, 45, 127, 114, 223, 112, 129, 72, 23, 187, 32,
    25, 135, 166, 35, 80, 182, 170, 145, 166, 195, 126, 36, 211, 163, 48, 235, 106, 17, 146, 209,
    13, 9, 34, 114, 206, 19, 165, 233, 157, 103, 122, 66, 83, 240, 223, 46, 203, 7, 170, 118, 160,
    111, 107, 64, 230, 159, 20, 25, 9, 192, 119, 242, 169, 143, 155, 224, 118, 13, 30, 175, 226,
    165, 35, 241, 250, 73, 219, 86, 140, 133, 50, 106, 74, 131, 132, 217, 221, 160, 122, 89, 230,
    23, 142, 80, 200, 171, 27, 231, 75, 94, 189, 100, 216, 234, 115, 167, 217, 190, 245, 73, 74,
    37, 183, 182, 180, 157, 83, 47, 14, 242, 247, 175, 9, 121, 75, 183, 48, 65, 74, 202, 95, 115,
    116, 68, 163, 95, 141, 113, 29, 96, 18, 162, 3, 82, 55, 162, 16, 240, 30, 157, 115, 8, 150,
    194, 38, 230, 0, 194, 2, 72, 177, 145, 242, 204, 115, 143, 42, 173, 248, 22, 24, 28, 139, 199,
    207, 137, 98, 214, 49, 13, 172, 8, 202, 131, 87, 235, 173, 15, 245, 235, 101, 125, 210, 24, 75,
    82, 3, 22, 87, 16, 255, 196, 16, 143, 142, 242, 123, 117, 133, 62, 44, 210, 50, 193, 172, 112,
    105, 141, 52, 210, 166, 249, 46, 157, 161, 237, 244, 155, 107, 228, 254, 108, 28, 43, 98, 227,
    117, 172, 119, 200, 125, 173, 209, 225, 223, 17, 130, 20, 62, 125, 98, 56, 138, 191, 241, 186,
    217, 100, 17, 191, 63, 163, 123, 135, 2, 43, 58, 116, 43, 143, 200, 39, 67, 215, 33, 143, 74,
    148, 103, 216, 211, 104, 81, 38, 128, 128, 187, 164, 145, 78, 37, 193, 172, 26, 165, 63, 216,
    74, 213, 154, 113, 227, 111, 42, 158, 193, 32, 102, 163, 153, 0, 176, 140, 69, 184, 9, 98, 232,
    254, 111, 27, 38, 47, 16, 240, 109, 203, 70, 190, 24, 73, 67, 244, 27, 116, 99, 114, 158, 63,
    113, 213, 221, 61, 1, 58, 173, 241, 44, 51, 114, 143, 116, 211, 64, 169, 198, 61, 107, 211,
    192, 84, 169, 245, 140, 178, 43, 114, 198, 169, 161, 55, 236, 73, 16, 253, 180, 150, 238, 248,
    17, 71, 126, 114, 100, 67, 121, 65, 9, 11, 181, 212, 175, 214, 139, 86, 189, 196, 237, 184,
    122, 29, 74, 205, 99, 140, 30, 49, 57, 168, 80, 50, 37, 71, 222, 42, 122, 192, 61, 9, 201, 101,
    239, 87, 249, 136, 241, 27, 16, 11, 235, 146, 139, 206, 167, 13, 78, 116, 137, 217, 184, 4,
    229, 78, 75, 144, 2, 129, 203, 88, 59, 147, 68, 204, 211, 195, 206, 84, 253, 87, 236, 18, 115,
    251, 81, 31, 82, 173, 41, 131, 18, 186, 72, 86, 181, 160, 167, 32, 235, 27, 10, 239, 154, 74,
    157, 211, 221, 89, 56, 196, 231, 16, 88, 5, 39, 40, 142, 242, 143, 50, 164, 122, 36, 123, 227,
    116, 134, 12, 160, 46, 87, 98, 138, 70, 202, 87, 53, 189, 30, 252, 79, 61, 28, 36, 242, 233, 5,
    108, 58, 174, 47, 168, 172, 190, 181, 65, 240, 181, 187, 184, 78, 71, 58, 202, 247, 109, 192,
    204, 211, 138, 250, 169, 189, 95, 133, 127, 29, 91, 75, 73, 246, 135, 220, 13, 176, 124, 25,
    56, 59, 67, 119, 204, 69, 57, 143, 18, 74, 91, 243, 197, 84, 193, 13, 232, 223, 193, 1, 135,
    241, 103, 171, 230, 167, 218, 61, 133, 65, 122, 126, 238, 94, 196, 15, 183, 148, 40, 181, 209,
    133, 145, 94, 213, 232, 74, 49, 180, 217, 66, 235, 102, 206, 10, 97, 177, 156, 194, 56, 119,
    22, 0, 11, 157, 179, 169, 76, 231, 130, 110, 105, 39, 247, 200, 116, 138, 238, 174, 92, 205,
    42, 158, 80, 119, 234, 68, 143, 85, 96, 98, 157, 146, 177, 172, 229, 33, 70, 31, 13, 194, 215,
    17, 245, 54, 25, 201, 113, 58, 207, 203, 221, 77, 132, 145, 52, 208, 150, 222, 117, 170, 42,
    150, 87, 215, 209, 118, 40, 169, 48, 195, 152, 211, 67, 200, 143, 103, 254, 232, 61, 211, 24,
    99, 120, 163, 149, 128, 57, 139, 170, 192, 230, 217, 219, 220, 241, 162, 129, 58, 88, 221, 31,
    46, 194, 77, 228, 178, 84, 220, 237, 254, 252, 139, 208, 214, 189, 35, 190, 229, 205, 103, 139,
    34, 208, 47, 83, 20, 118, 33, 252, 33, 250, 178, 79, 118, 255, 157, 209, 17, 181, 185, 84, 155,
    132, 159, 27, 17, 83, 247, 176, 13, 182, 9, 218, 178, 43, 10, 54, 196, 17, 24, 226, 15, 69,
    250, 198, 156, 69, 28, 0, 110, 79, 241, 78, 89, 154, 209, 92, 201, 20, 101, 148, 251, 52, 255,
    2, 95, 63, 219, 228, 223, 189, 68, 254, 241, 73, 12, 171, 64, 9, 91, 37, 207, 56, 33, 19, 44,
    232, 137, 241, 167, 66, 139, 246, 39, 15, 36, 158, 87, 117, 59, 29, 152, 2, 171, 45, 18, 13,
    81, 184, 178, 115, 251, 126, 86, 2, 56, 173, 187, 85, 172, 245, 69, 212, 127, 103, 146, 115,
    159, 106, 240, 131, 136, 145, 140, 236, 146, 148, 42, 13, 38, 49, 170, 207, 225, 28, 170, 216,
    241, 50, 69, 165, 147, 204, 115, 48, 203, 7, 189, 45, 134, 165, 106, 3, 239, 45, 224, 202, 221,
    171, 73, 46, 74, 86, 167, 255, 162, 1, 224, 33, 98, 110, 193, 221, 152, 127, 234, 83, 132, 9,
    25, 235, 10, 141, 141, 145, 6, 128, 245, 144, 170, 19, 5, 51, 226, 151, 219, 209, 231, 228,
    186, 146, 37, 67, 248, 4, 149, 71, 250, 126, 230, 206, 90, 127, 168, 32, 78, 172, 217, 8, 45,
    206, 178, 123, 95, 238, 214, 196, 176, 68, 250, 159, 89, 40, 239, 209, 106, 183, 148, 63, 179,
    231, 78, 13, 238, 62, 117, 81, 176, 217, 238, 228, 203, 198, 83, 213, 12, 141, 192, 156, 165,
    242, 205, 137, 30, 157, 244, 143, 177, 90, 83, 176, 107, 128, 164, 188, 163, 166, 46, 206, 196,
    31, 250, 28, 102, 20, 143, 53, 159, 251, 209, 44, 4, 108, 32, 118, 81, 223, 6, 219, 168, 79,
    133, 109, 8, 234, 154, 176, 149, 120, 192, 91, 46, 146, 119, 131, 237, 65, 163, 223, 164, 205,
    153, 167, 164, 49, 131, 157, 195, 111, 122, 246, 137, 78, 41, 216, 131, 162, 22, 122, 89, 199,
    130, 191, 65, 61, 175, 110, 170, 19, 96, 237, 40, 109, 206, 185, 178, 9, 61, 196, 177, 66, 76,
    67, 159, 189, 26, 99, 40, 125, 220, 60, 68, 120, 173, 135, 130, 141, 214, 192, 249, 76, 81,
    148, 221, 154, 254, 62, 187, 174, 61, 161, 146, 75, 155, 144, 49, 70, 135, 196, 191, 36, 88,
    184, 146, 162, 249, 134, 250, 188, 87, 101, 224, 240, 42, 127, 30, 143, 94, 92, 96, 14, 194,
    194, 44, 165, 197, 150, 234, 85, 132, 75, 9, 102, 53, 140, 131, 236, 39, 48, 17, 68, 187, 59,
    80, 58, 110, 112, 92, 186, 147, 70, 224, 5, 187, 225, 32, 77, 19, 56, 75, 134, 74, 249, 5, 28,
    211, 89, 60, 216, 71, 191, 42, 30, 77, 142, 231, 108, 27, 144, 90, 205, 54, 103, 126, 19, 90,
    84, 139, 111, 0, 147, 5, 93, 10, 72, 7, 136, 189, 100, 199, 186, 162, 155, 30, 203, 111, 198,
    103, 160, 178, 224, 16, 27, 72, 169, 41, 49, 28, 111, 50, 65, 222, 0, 132, 14, 83, 81, 132, 14,
    49, 36, 56, 165, 57, 67, 12, 229, 198, 152, 255, 91, 182, 41, 199, 106, 26, 31, 11, 229, 145,
    237, 99, 4, 26, 135, 228, 135, 83, 34, 200, 94, 26, 104, 92, 244, 87, 74, 114, 185, 116, 173,
    186, 35, 244, 26, 123, 187, 223, 5, 70, 88, 153, 208, 106, 108, 10, 90, 57, 168, 156, 223, 2,
    56, 166, 162, 230, 80, 170, 41, 181, 123, 193, 20, 54, 240, 32, 72, 54, 19, 153, 190, 27, 23,
    53, 246, 48, 206, 222, 53, 248, 157, 2, 101, 60, 154, 101, 132, 120, 63, 53, 148, 54, 76, 50,
    30, 213, 149, 134, 190, 138, 210, 240, 87, 22, 0, 182, 205, 241, 202, 72, 35, 156, 28, 160,
    123, 85, 180, 31, 22, 207, 152, 193, 202, 138, 91, 180, 27, 243, 139, 70, 255, 203, 31, 151,
    46, 238, 52, 18, 26, 176, 224, 214, 82, 48, 162, 16, 144, 46, 75, 19, 101, 93, 206, 34, 85,
    248, 81, 232, 87, 29, 57, 124, 198, 47, 191, 6, 108, 81, 146, 52, 192, 185, 3, 139, 104, 180,
    43, 134, 254, 161, 74, 149, 84, 213, 80, 220, 17, 112, 22, 37, 12, 110, 231, 33, 147, 35, 59,
    98, 198, 233, 1, 46, 13, 137, 252, 135, 103, 242, 227, 27, 56, 22, 58, 194, 48, 131, 4, 40,
    137, 197, 109, 202, 51, 228, 161, 254, 45, 46, 98, 73, 87, 55, 182, 250, 62, 252, 244, 150, 70,
    149, 158, 50, 35, 166, 226, 51, 172, 208, 95, 183, 96, 55, 185, 94, 48, 186, 5, 194, 19, 254,
    103, 96, 24, 148, 84, 113, 240, 47, 94, 214, 252, 53, 91, 249, 19, 90, 236, 96, 169, 69, 235,
    56, 154, 168, 129, 209, 196, 37, 195, 21, 144, 103, 124, 241, 171, 147, 237, 141, 223, 195,
    227, 242, 90, 95, 248, 187, 27, 96, 73, 74, 254, 195, 122, 225, 245, 249, 161, 92, 81, 240,
    108, 14, 235, 247, 33, 227, 6, 189, 33, 225, 120, 180, 121, 133, 87, 11, 102, 242, 192, 174,
    97, 185, 136, 124, 36, 252, 35, 244, 193, 215, 52, 20, 255, 1, 177, 179, 82, 106, 72, 97, 11,
    81, 30, 66, 88, 97, 110, 130, 59, 61, 151, 160, 220, 251, 255, 5, 36, 124, 148, 161, 165, 197,
    221, 222, 1, 48, 70, 90, 114, 156, 189, 202, 211, 220, 46, 53, 156, 192, 11, 24, 79, 91, 92,
    216, 240, 251, 24, 26, 52, 68, 140, 166, 192, 247, 251, 21, 54, 65, 119, 141, 144, 162, 194,
    216, 225, 231, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 13, 22, 32, 36, 44, 53, 64, 0,
];

const MESSAGE: [u8; 64] = [
    29, 204, 154, 221, 65, 56, 204, 167, 12, 130, 160, 47, 200, 7, 250, 159, 194, 82, 16, 98, 102,
    89, 177, 73, 39, 15, 105, 116, 233, 169, 31, 160, 5, 186, 46, 229, 44, 126, 198, 98, 20, 39,
    194, 203, 249, 170, 183, 196, 80, 111, 6, 194, 23, 17, 183, 73, 119, 95, 8, 123, 190, 17, 14,
    186,
];

const KEY_ID: KeyId = KeyId::KeyId1;

fn test_gen_key_pair() {
    let mut trng = unsafe {
        Trng::new(
            CsrngReg::new(),
            EntropySrcReg::new(),
            SocIfcTrngReg::new(),
            &SocIfcReg::new(),
        )
        .unwrap()
    };
    let mut entropy_gen = || trng.generate().map(|a| a.0);

    // This needs to happen in the first test
    CfiCounter::reset(&mut entropy_gen);

    let mut ml_dsa87 = unsafe { Mldsa87::new(MldsaReg::new()) };

    let mut hmac = unsafe { Hmac::new(HmacReg::new()) };
    let key_out_1 = KeyWriteArgs {
        id: KEY_ID,
        usage: KeyUsage::default().set_mldsa_seed_en(),
    };

    hmac.hmac(
        &HmacKey::from(&Array4x12::default()),
        &HmacData::from(&[0]),
        &mut trng,
        HmacTag::Key(key_out_1),
        HmacMode::Hmac384,
    )
    .unwrap();

    let seed = KeyReadArgs::new(KEY_ID);
    let public_key = ml_dsa87.key_pair(&seed, &mut trng).unwrap();
    assert_eq!(public_key, Mldsa87PubKey::from(PUBKEY));
}

fn test_sign() {
    let mut ml_dsa87 = unsafe { Mldsa87::new(MldsaReg::new()) };

    let mut trng = unsafe {
        Trng::new(
            CsrngReg::new(),
            EntropySrcReg::new(),
            SocIfcTrngReg::new(),
            &SocIfcReg::new(),
        )
        .unwrap()
    };

    let sign_rnd = Mldsa87SignRnd::default(); // Deterministic signing
    let seed = KeyReadArgs::new(KEY_ID); // Reuse SEED

    let signature = ml_dsa87
        .sign(
            &seed,
            &Mldsa87PubKey::from(PUBKEY),
            &MESSAGE.into(),
            &sign_rnd,
            &mut trng,
        )
        .unwrap();

    assert_eq!(signature, Mldsa87Signature::from(SIGNATURE));
}

fn test_verify() {
    let mut ml_dsa87 = unsafe { Mldsa87::new(MldsaReg::new()) };

    assert_eq!(
        ml_dsa87
            .verify(
                &Mldsa87PubKey::from(PUBKEY),
                &MESSAGE.into(),
                &Mldsa87Signature::from(SIGNATURE)
            )
            .unwrap(),
        Mldsa87Result::Success
    );
}

fn test_verify_failure() {
    let mut ml_dsa87 = unsafe { Mldsa87::new(MldsaReg::new()) };

    let msg = Mldsa87Msg::from([0xff; 64]);

    assert_eq!(
        ml_dsa87
            .verify(
                &Mldsa87PubKey::from(PUBKEY),
                &msg,
                &Mldsa87Signature::from(SIGNATURE)
            )
            .unwrap(),
        Mldsa87Result::SigVerifyFailed
    );
}

test_suite! {
    test_gen_key_pair,
    test_sign,
    test_verify,
    test_verify_failure,
}