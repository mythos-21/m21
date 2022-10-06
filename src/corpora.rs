
use std::env;
use rusqlite;


pub struct LEB {
    conn: rusqlite::Connection,
}


impl LEB {
    pub fn new() -> Self {
        let conn = rusqlite::Connection::open("/home/mythos21/repos/m21/data/bible/LEB.db").unwrap();
        LEB{conn}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_connect() {
        let leb = LEB::new();
        println!("Successfully connected to Lexham!");
    }
}


