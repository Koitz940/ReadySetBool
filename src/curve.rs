pub fn map(x: u16, y: u16) -> f64 {
    let n = (x as u64) | ((y as u64) << 16);
    let exponent: u64 = 0x3FF0_0000_0000_0000 | (n << 20);
    f64::from_bits(exponent) - 1.
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let x = n + 1.;
    let bits = x.to_bits();
    let important = ((bits >> 20) & 0xFFFF_FFFF) as u32;
    ((important & 0xFFFF) as u16, (important >> 16) as u16)
}

pub fn test_map() {
    for i in 0..=u16::MAX {
        for j in 0..=u16::MAX {
            let a = reverse_map(map(i, j));
            assert!(
                (i, j) == a,
                "Biyection didn't work for ({}, {}), got ({}, {})",
                i,
                j,
                a.0,
                a.1
            )
        }
    }
}
