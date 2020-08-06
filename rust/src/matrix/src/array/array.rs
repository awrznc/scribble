pub fn array(){
    const N: usize = 4;
    const HW: usize = 2;
    let a = [1.0; N];
    let b = [1.0; N];
    let mut c = [0.0; N];
    for i in 0..(HW) {
        for j in 0..(HW) {
            for k in 0..(HW) {
                c[i*HW+j] += a[i*HW+k] * b[k*HW+j];
            }
        }
    }
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
