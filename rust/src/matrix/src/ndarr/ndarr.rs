pub fn ndarr() {
    use ndarray::{Array, Array2};
    // 0.0 ~ 16.0 の行列を 1.0 刻みで作成
    // let array = Array2::range( 0.0, 16.0, 1.0 );
    let array: Array2<i16> = Array::from_shape_vec(
        (4, 4),
        (0..16).collect::<Vec<i16>>()
    ).unwrap();
    println!("{:?}", array);
}