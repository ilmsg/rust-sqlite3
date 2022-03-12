use sqlite::Error as sqErr;

pub struct User {
    fname: String,
}

#[derive(Debug)]
pub enum UserErr {
    DbErr(sqErr),
}

impl From<sqErr> for UserErr {
    fn from(s: sqErr) -> Self {
        UserErr::DbErr(s)
    }
}

impl User {
    pub fn add(&self, username: &str, password: &str) -> Result<(), UserErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut db = conn.prepare("insert into users (username, password) values (?, ?);")?;
        db.bind(1, username)?;
        db.bind(2, password)?;
        db.next()?;
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let db = User {
        fname: String::from("./data/data.db"),
    };

    match db.add("ryan", "top_secret") {
        Ok(_) => println!("Adding ryan was a success"),
        Err(UserErr::DbErr(ref err)) => println!(":( {:?}", err),
    }
}
