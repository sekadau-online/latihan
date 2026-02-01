use tracing::info;
// use tracing_subscriber;
use tracing_subscriber::fmt;
use latihan::math;

fn main() {
    // Init subscriber untuk menampilkan log (aman jika sudah inisialisasi)
    let _ = fmt().try_init();
    let y = 2;
    let mut a = 10;
    let mut b = a;
    let mut c = a;
    let mut d = a;
    let mut e = a;
    let f = a;
    info!("Nilai awal a = {a}, b = {b}, c = {c}", a=a, b=b, c=c);
    math::tambah(&mut a, y);
    math::kurang(&mut b, y);
    math::hasil(&mut c, y);
    math::kali(&mut d, y);
    math::bagi(&mut e, y);
    math::pangkat(f, y as u32);
}
