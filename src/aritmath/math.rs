use tracing::info;

pub fn tambah(a: &mut i32, y: i32) {
    *a += y;
    info!("a ditambah {y} = {a}");
}

pub fn kurang(b: &mut i32, y: i32){
    *b -= y;
    info!("b dikurang {y} = {b}");
}

pub fn hasil(c: &mut i32, y: i32){
    *c %= y;
    info!("c dibagi {y} sisa = {c}");
}

pub fn kali(d: &mut i32, y: i32){
    *d *= y;
    info!("d dikali {y} = {d}");
}

pub fn bagi(e: &mut i32, y: i32){
    *e /= y;
    info!("e dibagi {y} = {e}");
}

pub fn pangkat(base: i32, exp: u32) -> i32 {
    let result = base.pow(exp);
    info!("{base} dipangkatkan {exp} = {result}");
    result
}
