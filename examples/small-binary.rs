// #![feature(lang_items)]
// #![no_std]
#![no_main]

// extern crate alloc;
extern crate libc;

// #[panic_handler]
// fn panic_h(_info: &core::panic::PanicInfo) -> ! {
//     unsafe { libc::abort() }
// }

// #[lang = "eh_personality"]
// #[no_mangle]
// extern "C" fn eh_personality() {}

use libc::c_void;
use sqlstr::expr::{columns_iter, filter_where, from_table, lhs_binary_rhs, select, Cmp};
use sqlstr::{SqlCommand, SqlExpr, Void};

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let sql = make_sqlstr();

    let written = unsafe {
        libc::write(
            libc::STDOUT_FILENO,
            sql.as_command().as_ptr() as *const c_void,
            sql.as_command().len(),
        )
    };

    if written == sql.as_command().len() as isize {
        0
    } else {
        -1
    }
}

fn make_sqlstr() -> SqlCommand<Void> {
    let mut sql: SqlCommand<Void> = SqlCommand::default();

    select(&mut sql);
    columns_iter(&mut sql, ["name", "email"]);

    from_table(&mut sql, "user");

    filter_where(&mut sql);
    // SAFETY: infallible error
    unsafe {
        lhs_binary_rhs(
            &mut sql,
            SqlExpr::<&str>::Expr("email"),
            Cmp::Eq,
            SqlExpr::Value("crab@email.com"),
        )
        .unwrap_unchecked();
    }

    sql
}
