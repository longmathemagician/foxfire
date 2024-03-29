#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("resources/icons/foxfire.ico", "application_icon");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {}
