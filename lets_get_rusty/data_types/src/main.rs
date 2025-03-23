fn main() {
    // All are Scalar types below: (as opposed to Compound Types)

        // Integers

        let a = 98_222; // Decimal
        let b = 0xff; // Hex
        let c = 0o77; // Octal
        let d = 0b1111_0000; // Binary
        let e = b'A'; // Byte (u8 only)
        let f: u8 = 255; // integer overflow at 256 (wraps to 0, 257 becomes 1)

        // Floating-point numbers (numbers with decimal points)

        let f = 2.0;
        let g: f32 = 3.0;
        // addition
        let sum = 5 + 10;
        //subtraction
        let difference = 95.5 - 4.3;
        //multiplication
        let product = 4 * 30;
        //division
        let quotient = 56.7 / 32.2;
        // remainder
        let remainder = 43 % 5;

        // Booleans

        let t = true; 

        let f = false;

        // Character

        let c = 'z';
        let z = '$';

// All the following are Compound Types (as opposed to scalars)

// Tuples
    let tup = ("First job here I come!", 150_000);
    let (goal, first_job_salary) = tup;
    let first_job_salary = tup.1;

//Arrays (are always fixed length, as opposed to Vectors, which can change)
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];



}
