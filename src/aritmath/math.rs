use tracing::info;

pub fn tambah(a: &mut i32, y:i32) {
    *a += y;
    info!("a ditambah {y} = {a}", y=y, a=a);
}

pub fn kurang(b: &mut i32, y:i32){
    *b -= y;
    info!("b dikurang {y} = {b}", y=y, b=b);
}

pub fn hasil(c: &mut i32, y:i32){
    *c %= y;
    info!("c dibagi {y} sisa = {c}", y=y, c=c);
}
