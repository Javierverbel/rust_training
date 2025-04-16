pub fn f1(pair: &mut (u8, u8), flag: bool) -> &mut u8 {
    if flag { &mut pair.0 } else { &mut pair.1 }
}

pub fn f2(tuple: &mut [u32], number: usize) -> &mut u32 {
    &mut tuple[number]
}

pub fn f3(tuple: &mut [u32], number: usize) -> &mut u32 {
    &mut tuple[tuple.len() - number - 1]
}

pub fn f4(tuple: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let len_subarrays = tuple.len() / 4;
    (
        &tuple[..len_subarrays],
        &tuple[len_subarrays..2 * len_subarrays],
        &tuple[2 * len_subarrays..3 * len_subarrays],
        &tuple[3 * len_subarrays..],
    )
}

#[cfg(test)]
mod test {
    use super::{f1, f2};

    #[test]
    fn test_f1() {
        let pair = &mut (4u8, 8u8);
        // let mut pair2 = &(4u8, 8u8);
        // let mut pair3 = &(4u8, 8u8);
        // pair2 = pair3;

        let flag = true;
        let x = f1(pair, flag);
        *x = 10;
        assert_eq!(pair.0, 10u8);
    }

    #[test]
    fn test_f2() {
        let tuple = &mut [1u32, 2u32, 3u32, 4u32, 5u32, 6u32];

        let x = f2(tuple, 5);
        *x = 20;
        assert_eq!(tuple[5], 20);
    }

    #[test]
    fn test_f3() {}
}
