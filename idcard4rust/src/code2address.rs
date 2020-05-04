use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

fn loadBook() -> HashMap<String, String, RandomState> {
    let p = Path::new("../idcard.txt");
    let mut file = File::open(p).unwrap();
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    let mut res = content.split_whitespace();
    let mut a: HashMap<String, String> = HashMap::new();
    loop {
        let keyOpt = res.next();
        if keyOpt.is_none() {
            break;
        }
        let key = keyOpt.unwrap();
        let value = res.next().unwrap();
        a.insert(key.to_string(), value.to_string());
    }
    a
}

pub fn getBook() -> &'static HashMap<String, String, RandomState> {
    static mut BOOK: Option<Arc<HashMap<String, String>>> = None;
    unsafe {
        BOOK.get_or_insert_with(|| {
            Arc::new(loadBook())
        });
        BOOK.as_ref().unwrap()
    }
}
