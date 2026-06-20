fn main() {
    let val: f64 = 10.87;
    let fbits: u64 = val.to_bits();
    println!("binary:   {0:#064b}", fbits);
    let r: FloatComponents = deconstruct(val);
    println!("sign:     {0:#b} ({0})", r.sign);
    println!("exponent: {0:#011b} ({0})", r.exponent);
    println!("offset:   {0:#0b} ({0})", r.offset);
}

struct FloatComponents {
    sign: u64,
    exponent: u64,
    offset: u64
}

fn deconstruct(n: f64) -> FloatComponents {
    let sign_mask: u64 =     0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let exponent_mask: u64 = 0b0111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let offset_mask: u64 =   0b1000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

    let bits: u64 = n.to_bits();

    let result: FloatComponents = FloatComponents {
        sign: (bits & sign_mask) >> 63,
        exponent: (bits & exponent_mask) >> 52,
        offset: bits & offset_mask
    };
    return result;
}

