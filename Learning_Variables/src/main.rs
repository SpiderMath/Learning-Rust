fn main() {
    println!("Hello {}", "SpiderMath");
    // Using Templates -> {} replaced by the Second string/variable

    println!("Hello {}, in {}", "SpiderMath", "Atom");
    // Works -> So basically it is kinda like an Array of {} to be replaced by String[]

    let x = 45;

    println!("The value of x is {}", x);

    // x = 60 -> error, since on declaration, the variables are immutable (kinda like const)

    let mut y = 420;

    println!("The value of y is {}", y);

    y = 69;

    println!("The new value of y is {}", y);
    // Basically to get "let" or "var" like property, you need do use "let mut" OOF

    /*
        For INTEGERS, there are 2 main divisions,
            - Signed -> Use this if you don't know what the sign of the integer might be (maybe + or -)
            - Unsigned -> Use this if you know that the sign of the integer is + (always)
        There are a bunch of sizes,
            - 8 bit
            - 16 bit
            - 32 bit
            - 64 bit
            - 128 bit
            - arch -> Depends on the OS, whether 32-bit or 64-bit
        Signed n-bit integers (in) -> Upper Limit: (2^(n - 1)) - 1 Lower Limit: -2^(n - 1)
        Unsigned n-bit integers (un) -> (2^n) - 1

        Integers default to i32 -> Fastest in all systems, even in 64 bit

        Float types -> f32 & f64
        Default is f64 since it is as fast as f32 in most places and has more precision

        Boolean -> tru or false lol, nothing new

        Char -> Single Quotes (RIP JS conventions for most lul)
        String -> Double Quotes, type is &str dayummmmmm
    */
    let str: &str = "str";
    let char: char = 'e';

    println!("This is a string \"{}\"", str);
    println!("This is a char '{}'", char);
}
