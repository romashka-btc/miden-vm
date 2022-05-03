use super::{build_test, Felt};

fn mac(a: u32, b: u32, c: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + (b as u64 * c as u64) + carry as u64;
    ((tmp >> 32) as u32, tmp as u32)
}

#[test]
fn test_mac() {
    let source = "
    use.std::math::secp256k1

    begin
        exec.secp256k1::mac
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let (hi, lo) = mac(
        stack[3] as u32,
        stack[2] as u32,
        stack[1] as u32,
        stack[0] as u32,
    );

    let test = build_test!(source, &stack);
    test.expect_stack(&[hi as u64, lo as u64]);
}

fn u256xu32(a: &mut [u32], b: u32, c: &[u32]) {
    assert_eq!(a.len(), 9);
    assert_eq!(c.len(), 8);

    let mut carry: u32;

    let v = mac(a[0], b, c[0], 0);
    carry = v.0;
    a[0] = v.1;

    let v = mac(a[1], b, c[1], carry);
    carry = v.0;
    a[1] = v.1;

    let v = mac(a[2], b, c[2], carry);
    carry = v.0;
    a[2] = v.1;

    let v = mac(a[3], b, c[3], carry);
    carry = v.0;
    a[3] = v.1;

    let v = mac(a[4], b, c[4], carry);
    carry = v.0;
    a[4] = v.1;

    let v = mac(a[5], b, c[5], carry);
    carry = v.0;
    a[5] = v.1;

    let v = mac(a[6], b, c[6], carry);
    carry = v.0;
    a[6] = v.1;

    let v = mac(a[7], b, c[7], carry);
    a[8] = v.0;
    a[7] = v.1;
}

#[test]
fn test_u256xu32() {
    let source = "
    use.std::math::secp256k1

    proc.wrapper.2
        push.3102021493.1265174470.1329925018.4146020526
        popw.local.0
        push.1873376618.982499173.390191265.153760297
        popw.local.1

        push.env.locaddr.0
        movdn.9
        push.env.locaddr.1
        movdn.10

        exec.secp256k1::u256xu32
    end

    begin
        exec.wrapper
    end";

    let stack = [
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
        rand_utils::rand_value::<u64>() as u32 as u64,
    ];

    let mut a = [
        stack[8] as u32,
        stack[7] as u32,
        stack[6] as u32,
        stack[5] as u32,
        stack[4] as u32,
        stack[3] as u32,
        stack[2] as u32,
        stack[1] as u32,
        0,
    ];
    let b = stack[0] as u32;
    let c = [
        4146020526, 1329925018, 1265174470, 3102021493, 153760297, 390191265, 982499173, 1873376618,
    ];

    u256xu32(&mut a, b, &c);

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    for i in 0..9 {
        assert_eq!(Felt::new(a[i] as u64), strace[i]);
    }
}
