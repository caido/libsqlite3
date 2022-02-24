use libsqlite3_sys::sqlite3_auto_extension;

mod bindings;

pub fn init() {
    unsafe {
        // Init SHA1
        let sha_init = bindings::sqlite3_sha_init as *const ();
        sqlite3_auto_extension(Some(
            std::mem::transmute::<*const (), unsafe extern "C" fn()>(sha_init),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn can_open_database() {
        let conn = Connection::open_in_memory().unwrap();

        let mut stmt = conn.prepare("SELECT 1").unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let value: i64 = row.get(0).unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn can_execute_sha1_database() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let mut stmt = conn.prepare("SELECT upper(sha1(1))").unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let value: String = row.get(0).unwrap();
        assert_eq!(value, "356A192B7913B04C54574D18C28D46E6395428AB");
    }
}
