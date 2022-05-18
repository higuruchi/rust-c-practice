#[link(name="foo", kind="static")]
extern "C" {
    fn foo();
    fn hoge(h: *const Hoge);
    fn fuga() -> Hoge;
}

#[repr(C)]
struct Hoge {
    x: i32,
    y: i32
}

fn main() {
    unsafe { foo(); }
    let h = Hoge{x: 1, y: -2};
    unsafe { hoge(&h as *const Hoge) };
    let got_h = unsafe{ fuga() };
    println!("got_h x: {}, y: {}", got_h.x, got_h.y);
}
