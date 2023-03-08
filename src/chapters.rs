pub mod ch1 {
    use std::fmt::{self, Formatter, Display};

    /// prints basic hello world example
    pub fn hello_world() {
        // this is a comment
        /*
        this
        is
        a
        multi-line
        comment!
         */
        println!("Hello, World!");
        println!("I'm a Rustacean!")
    }

    pub fn comments() {
        let x = 5 +  90 +  5;
        println!("Is `x` 10 or 100? drum roll.... x = {}", x);
    }

    pub fn formatted_print() {
        println!("{} days", 31);

        println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

        println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
        
        println!("Base 10:               {}",   69420); //69420
        println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
        println!("Base 8 (octal):        {:o}", 69420); //207454
        println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
        println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C

        println!("{number:>5}", number=1);
        println!("{number:0<5}", number=1);
        println!("{number:0>width$}", number=1, width=5);
        println!("My name is {0}, {1} {0}", "Bond", "James");

        #[allow(dead_code)]
        struct Structure(i32);

        // This will not compile because `Structure` does not implement
        // fmt::Display
        // println!("This struct `{}` won't print...", Structure(3));
        // TODO ^ Try uncommenting this line

        let number: f64 = 1.0;
        let width: usize = 5;
        println!("{number:>width$}");

        /*
        Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. 
        For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. 
        (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
         */
        let pi = 3.141592;
        println!("Pi is roughly {:.3}", pi);

    }

    pub fn debug() {
        #[derive(Debug)]
        struct Structure(i32);

        #[derive(Debug)]
        struct Deep(Structure);

        println!("{:?} months in a year", 12);
        println!("{1:?} {0:?} is the actor {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");
        
        println!("Now {:?} will print!", Structure(3));

        println!("How can we print just 7? For example deep is: {:?}", Deep(Structure(7)));

    }

    pub fn debug_pretty() {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8
        }

        let name = "Peter";
        let age = 27;
        let peter = Person { name, age};

        println!("{:#?}", peter);
    }

    pub fn display_122_1() {
        struct Structure(i32);

        impl fmt::Display for Structure {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }

    pub fn display_122_2() {
        #[derive(Debug)]
        struct MinMax(i64, i64);

        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }

        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "x: {}, y:{}", self.x, self.y)
            }
        }

        #[derive(Debug)]
        struct Complex {
            real: f64,
            imag: f64, 
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "real: {}, imag: {}", self.real, self.imag)
            }
        }

        example();

        fn example() {
            let minmax = MinMax(0, 14);

            println!("Compare structures:");
            println!("Display: {}", minmax);
            println!("Debug: {:?}", minmax);

            let big_range = MinMax(-300, 300);
            let small_range = MinMax(-3,3);

            println!("The big range is {big} and the small is {small}",
                small = small_range,
                big = big_range);

            let point = Point2D { x: 3.3, y: 7.2 };

            println!("Compare points:");
            println!("Display: {}", point);
            println!("Debug: {:?}", point);

            // not really sure if they wanted to actually use complex numbers or just fancy printing. oh well.
            let complex = Complex{ real: 3.3, imag: 7.2};
            println!("Display: {0} + {1}i", complex.real, complex.imag);
            println!("Debug: {:?}", complex);

            // Error. Both `Debug` and `Display` were implemented, but `{:b}`
            // requires `fmt::Binary` to be implemented. This will not work.
            // println!("What does Point2D look like in binary: {:b}?", point);
        }
    }

    pub fn testcase_list() {
        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let vec = &self.0;

                write!(f, "[")?;

                for (count, v) in vec.iter().enumerate() {
                    if count != 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}",count, v)?;
                }

                write!(f, "]")
            }
        }

        let v = List(vec![1,2,3]);
        println!("{}", v)

    }

    pub fn foratting_guide() {

    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument)
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    // activity
    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            // this threw me for a loop for a bit
            let hexvalue = format!("{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue);
            write!(f, "RGB ({}, {}, {}) 0x{}",
                self.red, self.green, self.blue, hexvalue)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }

    }

}

pub mod ch2 {
    pub fn literals_and_operators() {
        println!("1 + 2 = {}", 1u32 + 2);

        println!("1 - 2 = {}", 1i32 -2);
        // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
        // needs signed, otherwise it will overflow to highest u32

        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);
        
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        println!("One million is written as {}", 1_000_000u32);
    }

    pub fn tuples_lesson() {
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            let (int_param, bool_param) = pair;

            (bool_param, int_param)
        }

        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);

        let long_tuple = (1u8, 2u16, 3u32, 4u64, 
            -1i8, -2i16, -3i32, -4i64,
            0.1f32, 0.2f64,
            'a', true);
        
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // But long Tuples (more than 12 elements) cannot be printed
        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);
        // TODO ^ Uncomment the above 2 lines to see the compiler error

        let pair = (1, true);
        println!("pair is {:?}", pair);

        println!("the reversed pair is {:?}", reverse(pair));

        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        let tuple = (1, "hello", 4.5, true);

        let (a,b,c,d) = tuple;

        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);

    }

    pub fn arrays_and_slices() {
        use std::mem;

        fn analyze_slice(slice: &[i32]) {
            println!("first element of the slice: {}", slice[0]);
            println!("the slice has {} elements", slice.len());
        }

        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let ys: [i32; 500] = [0; 500];

        // really just wanted to analyze this one
        analyze_slice(&ys);
        println!("The 128th element is {}", ys[128]);

        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        println!("number of elements in array: {}", xs.len());
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        println!("borrow a section of the array as a slice");
        analyze_slice(&ys[1 .. 4]);

        let empty_array: [u32; 0] = [];
        assert_eq!(&empty_array, &[]);
        assert_eq!(&empty_array, &[][..]);

        for i in 0..xs.len() + 1 {
            match xs.get(i) {
                Some(xval) => println!("{}: {}", i, xval),
                None => println!("Slow down! {} is too far!", i),
            }
        }

    }

    pub fn structures() {
        
    }

}