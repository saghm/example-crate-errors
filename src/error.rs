error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Parse(::std::num::ParseIntError);
    }

   errors {
        DivideByZero(i: i32) {
            description("cannot divide by 0")
            display("cannot divide {} by 0", i)
        }

        InvalidInput(s: String) {
            description("input contains non-integers")
            display("input '{}' contains non-integers", s)
        }
    }
}
