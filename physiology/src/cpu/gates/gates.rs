pub mod gates {
    pub fn not(a: u8) -> u8 {
        if a == 0 { 1 } else { 0 }
    }

    pub mod and {
        pub fn two(a: u8, b: u8) -> u8 {
            a & b
        }

        pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
            a & b & c & d
        }

        pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
            a & b & c & d & e & f & g & h
        }
    }

    pub mod or {
        pub fn two(a: u8, b: u8) -> u8 {
            a | b
        }

        pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
            a | b | c | d
        }

        pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
            a | b | c | d | e | f | g | h
        }
    }

    pub mod nand {
        use super::not;

        pub fn two(a: u8, b: u8) -> u8 {
            not(a) & not(b)
        }

        pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
            not(a) & not(b) & not(c) & not(d)
        }

        pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
            not(a) & not(b) & not(c) & not(d) & not(e) & not(f) & not(g) & not(h)
        }
    }

    pub mod nor {
        use super::not;

        pub fn two(a: u8, b: u8) -> u8 {
            not(a) | not(b)
        }

        pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
            not(a) | not(b) | not(c) | not(d)
        }

        pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
            not(a) | not(b) | not(c) | not(d) | not(e) | not(f) | not(g) | not(h)
        }
    }

    pub fn xor(a: u8, b: u8) -> u8{
        let not_a = not(a);
        let not_b = not(b);
        let and1 = and::two(not_a, b);
        let and2 = and::two(a, not_b);
        or::two(and1, and2)
    }

    pub fn xnor(a: u8, b: u8) -> u8 {
        let not_a = not(a);
        let not_b = not(b);
        let and1 = and::two(a, b);
        let and2 = and::two(not_a, not_b);
        or::two(and1, and2)
    }
}