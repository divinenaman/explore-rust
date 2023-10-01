
mod once_cell_script;

use once_cell_script::{DB, test_db};


fn do_something1 () -> String {
    DB::get_data().to_owned()
}

fn do_something2 () -> String {
    DB::get_data().to_owned()
}

fn main () {
    DB::init("DATA");

    let res1 = do_something1();
    let res2 = do_something2();
    let res3 = test_db::test_something();

    println!("{}, {}, {}", res1, res2, res3);
}
