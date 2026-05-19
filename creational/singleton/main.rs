use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;

static INSTANCE: LazyLock<Arc<Mutex<Singleton>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Singleton { value: "foobar" })));

struct Singleton {
    value: &'static str,
}

impl Singleton {
    pub fn get<'a>() -> MutexGuard<'a, Singleton> {
        INSTANCE.lock().unwrap()
    }
}

fn main() {
    let mut s1 = Singleton::get();

    assert_eq!(s1.value, "foobar");

    s1.value = "barbaz";
    drop(s1);

    let s2 = Singleton::get();

    assert_eq!(s2.value, "barbaz");
}
