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
    use super::and;

    pub fn two(a: u8, b: u8) -> u8 {
        not(and::two(a, b))
    }

    pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
        not(and::four(a, b, c, d))
    }

    pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
        not(and::eight(a, b, c, d, e, f, g, h))
    }
}

pub mod nor {
    use super::not;
    use super::or;

    pub fn two(a: u8, b: u8) -> u8 {
        not(or::two(a, b))
    }

    pub fn four(a: u8, b: u8, c: u8, d: u8) -> u8 {
        not(or::four(a, b, c, d))
    }

    pub fn eight(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u8 {
        not(or::eight(a, b, c, d, e, f, g, h))
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
    not(xor(a, b))
}