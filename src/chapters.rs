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

}

pub mod ch3 {
    pub fn structures() {
        #[allow(dead_code)]

        #[derive(Debug)]
        struct Person {
                name: String,
                age: u8,
        } 
 
        struct Unit;
 
        struct Pair(i32, f32);
 
        struct Point {
         x: f32,
         y: f32,
        }
 
        struct Rectangle {
             top_left: Point,
             bottom_right: Point,
        }
 
        let name = String::from("Peter");
        let age = 27;
        let peter =  Person { name, age };
 
        println!("{:?}", peter);
 
        let point: Point = Point { x: 10.3, y: 0.4};
        println!("point coordinates: ({}, {})", point.x, point.y);
 
        let bottom_right = Point {x: 5.2, ..point};
        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
 
        let Point { x: left_edge, y: top_edge } = point;
 
        let rectangle = Rectangle {
             top_left: Point { x: left_edge, y: top_edge },
             bottom_right: bottom_right,
        };
 
        let _unit = Unit;
        
        let pair = Pair(1, 0.1);
        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        let Pair(integer, decimal) = pair;
        println!("pair contains {:?} and {:?}", integer, decimal);

        //activity
        impl Rectangle {
            fn rect_area(&self) -> f32 {
                self.top_left.x * self.bottom_right.y
            }
        }
        println!("Rectangle with points top-left: {} and bottom-right: {} has an area of {}", 
            rectangle.top_left.x, rectangle.bottom_right.y, rectangle.rect_area());

        // not using a borrow feels wrong
        impl Point {
            fn square(self, width: f32) -> Rectangle {
                let point = Point{
                    x: self.x + width,
                    y: self.y + width,
                };
                Rectangle { top_left: self, bottom_right: point }
            }
        }

        println!("Point has coords x:{}, y: {}.", point.x, point.y);
        let new_square = point.square(5.1);
  
        println!("Creating a square, by adding 5.1: bottom-right x: {}, bottom-right y: {}", new_square.bottom_right.x, new_square.bottom_right.y);
        println!("This rectangle/square has an area of: {}", new_square.rect_area());


    }

    pub fn enum_nom_nom() {
        enum WebEvent {
            PageLoad,
            PageUnload,
            KeyPress(char),
            Paste(String),
            Click { x: i64, y: i64 },
        }

        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
                WebEvent::Paste(s) => println!("pasted \"{}\".", s),
                WebEvent::Click { x, y } => {
                    println!("clicked at x={}, y={}.", x, y);
                },
            }
        }

        let pressed = WebEvent::KeyPress('x');
        let pasted  = WebEvent::Paste("my text".to_owned());
        let click   = WebEvent::Click { x: 20, y: 80 };
        let load    = WebEvent::PageLoad;
        let unload  = WebEvent::PageUnload;
    
        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

    }

    pub fn enums_use() {
        #![allow(dead_code)]

        enum Status {
            Rich,
            Poor
        }

        enum Work {
            Civilian,
            Soldier,
        }

        // this code works fine in main() but not so much in this mod file, so had to remove `crate` prepend
        use Status::{Poor, Rich};
        use Work::*;

        let status = Poor;
        let work = Civilian;

        // let status = Status::Poor;
        // let work = Work::Civilian;

        match status {
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }

        match work {
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldiers fight!"),
        }


    }

    pub fn c_like() {
        #![allow(dead_code)]

        enum Number {
            Zero,
            One,
            Two,
        }

        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);
        
        println!("roses are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }

    pub fn testcase_linked_list() {
        use List::*;

        enum List {
            Cons(u32, Box<List>),
            Nil,
        }

        impl List {
            fn new() -> List {
                Nil
            }

            fn prepend(self, elem: u32) -> List {
                Cons(elem, Box::new(self))
            }

            fn len(&self) -> u32 {
                match *self {
                    Cons(_, ref tail) => 1 + tail.len(),
                    Nil => 0
                }
            }

            fn stringify(&self) -> String {
                match *self {
                    Cons(head, ref tail) => {
                        format!("{}, {}", head, tail.stringify())
                    },
                    Nil => {
                        format!("Nil")
                    }
                }
            }
        }

        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());



    }

    pub fn constants() {
        static LANGUAGE: &str = "Rust";
        const THREASHOLD: i32 = 10;

        fn is_big(n: i32) -> bool {
            n > THREASHOLD
        }

        let n = 16;
        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THREASHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else {"small"});

        // THRESHOLD = 5;
        // FIXME ^ Comment out this line
    }

}

pub mod ch4 {
    pub fn mutability() {
        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);

        mutable_binding += 1;
        println!("After mutation: {}", mutable_binding);

        //_immutable_binding += 1;
        // Fix ^
    }

    pub fn scope_and_shadow() {
        let long_live_the_binding = 1;

        {
            let short_lived_binding =2;

            println!("inner short: {}", short_lived_binding);
        }

        // println!("outer short: {}", short_lived_binding);
        // FIX ^
        
        println!("outer long: {}", long_live_the_binding);

        let shadow_and_bind = 1;

        {
            println!("before being shadowed: {}", shadow_and_bind);

            let shadow_and_bind = "abc";

            println!("shadowed inner block: {}", shadow_and_bind);
        }

        println!("outside inner block: {}", shadow_and_bind);

        let shadow_and_bind = 2;
        println!("shadowed in outer block: {}", shadow_and_bind);
    }

    pub fn declare_first() {
        // Generally considered seldom used
        let a_binding;

        {
            let x = 2;

            a_binding = x * x;

        }

        println!("a binding: {}", a_binding);

        let another_binding;

        // println!("another binding: {}", another_binding);
        // FIXME ^ -- doesn't work
        
        another_binding = 1;
        println!("another binding: {}", another_binding);
    }

    pub fn freezing() {
        let mut _mutable_integer = 7i32;
        {
            let _mutable_integer = _mutable_integer;
            
            // _mutable_integer = 50;
            // FIXME ^ -- it's frozen, can't mut

        }

        _mutable_integer = 3;
    }

}

pub mod ch5 {

    pub fn casting() {
        #![allow(overflowing_literals)]

        let decimal = 65.4321_f32;
        // let integer: u8 = decimal;
        // FIXME ^ you can't do this.

        let integer = decimal as u8;
        let character = integer as char;

        // let character = decimal as char;
        // FIXME ^ float can't directly convert to char

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        println!("1000 as u16 is: {}", 1000 as u16);
        println!("Did you know? 1000 as u8 is: {}", 1000 as u8);
        println!("Overflowing is fun! Here's -1i8 as u8 is: {}", (-1i8) as u8);

        println!("1000 mod 256 is: {}", 1000 % 256);

        println!("128 as a i16 is: {}", 128 as i16);
        println!("128 as a i8 is: {}", 128 as i8);

        println!("1000 as a u8 is: {}", 1000 as u8);
        println!("232 as a i8 is: {}", 232 as i8);

        println!("300.0 as u8 is: {}!", 300.0_f32 as u8);
        println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
        println!("nan as u8 is: {}", f32::NAN as u8);

        unsafe {
            println!("Those of you with weak constitutions may want to leave...");
            println!("Too late!!");

            println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
            println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
            println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
        }

    }

    pub fn literals() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        let i = 1;
        let f = 1.0;

        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    pub fn inference() {
        let elem = 5u8;
        
        let mut vec = Vec::new();

        vec.push(elem);

        //- TODO ^ try commenting this out
        // need to define T in Vec

        println!("{:?}", vec);
    }

    pub fn aliasing() {
        type NanoSecond = u64;
        type Inch = u64;
        type U64 = u64;

        let nanoseconds: NanoSecond = 5 as U64;
        let inches: Inch = 2 as U64;

        println!("{} nanoseconds + {} inches = {} units?",
            nanoseconds,
            inches,
            nanoseconds + inches);
    }

}

