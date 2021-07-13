fn main() {
    println!("== TYPE INFERENCE");
    // == Type inference
    let number = "42";
    let number: u32 = number.parse().expect("That's not a number!");

    // `u32` is required in the reassigment above to let `parse()`
    // infer which type the result should be

    println!("Number is: {}", number);

    println!("");
    println!("= SCALAR TYPES");
    // = Scalar types
    // Rust has four primary scalar types: integers, floating-point
    // numbers, Booleans, and characters.

    // == Integer types
    // Length       Signed  Unsigned
    // 8-bit        i8      u8
    // 16-bit       i16     u16
    // 32-bit       i32     u32
    // 64-bit       i64     u64
    // 128-bit      i128    u128
    // arch         isize   usize

    // == Integer literals
    // = Number literals =      = Example =
    // Decimal                  98_222
    // Hex                      0xff
    // Octal                    0o77
    // Binary                   0b1111_0000
    // Byte (u8 only)           b'A'

    // Example of integer in code

    let x = 25_000;
    println!("Value of 25_000 is: {}", x);

    let x = 0xff;
    println!("Value of 0xff is: {}", x);

    // === Integer overflow
    // Because the maximum value of a `u8` is 255, when the below code
    // is run, it causes an overflow. In debug mode, this causes the
    // program to panic and crash. When compiled in `--release` mode,
    // these checks are not included and so the program keeps running.
    // In the case of `--release` mode, the numbers will simply wrap
    // according to two's complement. 256 becomes 0, 257 becomes 1...
    //
    // Depending on this wrapping behaviour is considered an error and
    // you should instead use `Wrapping` from the standard library
    // instead to do explicit wrapping.

    let x: u8 = 255;
    // x = x + 1; // Comment out to let program run in debug mode
    println!("Value of x is: {}", x);

    println!("");
    println!("== FLOATING-POINT TYPES");
    // == Floating-point types
    // Length   Name
    // 32-bit   f32
    // 64-bit   f64
    //
    // === Why floating-point types?
    // While integers can only represent whole numbers like {1, 2, 3},
    // floating-point numbers can represent values such as {1.1, 2.7, 3.14}.

    const PI: f64 = 3.1415926535897932384626433832;
    println!("Value of PI is: {}", PI);

    let x = 2.0; // Defaults to `f64`
    println!("Value of x is: {}", x);

    let y: f32 = 3.0;
    println!("Value of y is: {}", y);

    // == Numeric operations

    #[allow(unused)]
    {
        // addition
        let sum = 6 + 9;

        // subtraction
        let difference = 9 - 6;

        // multiplication
        let product = 6 * 9;

        // division
        let quotient = 6.0 / 9.0;

        // remainder / modulo
        let remainder = 43 % 5;
    }

    println!("");
    println!("== Character type");
    // == Character type
    // In rust, the character type is 4 bytes and represents a unicode
    // code point.

    let c = 'c';
    let z = 'z';
    let tm_symbol = '™';

    println!("c = {}", c);
    println!("z = {}", z);
    println!("tm_symbol = {}", tm_symbol);

    println!("");
    println!("== TUPLE TYPE"); 
    // == Tuple type

    let tup = (400, 2.4, -1); // Constructing tuple with inferred types
    let tup: (u32, f32, i8) = tup; // Constructing tuple with given types
    let (x, y, z) = tup; // Destructuring tuple into variables x, y, and z
    println!("Tuple (x, y, z) = ({}, {}, {})", x, y, z);

    // It is also possible to access each element in the tuple by its index:
    println!("Value of tup.0 = {}", tup.0);

    println!("");
    println!("== ARRAY TYPE");
    // == Array type
    // A different way of collecting multiple values into one variable.
    // Unlike a tuple, all values in an array must be of the same type.

    let a = [1, 2, 3, 4]; // Array with inferred type
    let a: [u8; 4] = a; // Array with given type
    println!("a = {:?}", a); // Print entire array with `{:?}`
    println!("a[0] = {}", a[0]); // Access specific index of array

    // let index = 4;
    // let element = a[index];
    // println!("a[4] = {}", element); // Caught at compile time or causes a panic
}
