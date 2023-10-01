// once_cell test

use once_cell::sync::OnceCell;

pub struct DB {
    store: String
}


pub mod test_db {
    use crate::once_cell_script::DB;

    pub fn test_something () -> String {
        DB::get_data().to_owned()
    }
}

impl DB {
    pub fn get_data() -> &'static str {
        let db = INSTANCE.get().expect("DB not initialized!");
        &db.store
    }

    pub fn init(data: &str) -> () {
        let _ = INSTANCE.set(DB {
            store: data.to_owned()
        });
    }
}

static INSTANCE: OnceCell<DB> = OnceCell::new();
