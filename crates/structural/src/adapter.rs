//Adapter:
//Convert the interface of the class into another interface clients expect.
// ┌─────────────┐         ┌───────────────┐               ┌──────────────────┐
// │  Client     ├─────────►  Target       │               │                  │
// └─────────────┘         ┌───────────────┤               │   Adaptee        │
//                         │               │               ├──────────────────┤
//                         │   Request()   │               │                  │
//                         └─────────────▲─┘               │  SpecificRequest()
//                                       │                 └─────────▲─────────
//                                     ┌─┴───────────────┐           │
//                                     │   Adapter       ├─implement─┘
//                                     ├─────────────────┤
//                                     │                 │        ┌──────────────
//                                     │   Request()     ├────────►  SpecificRequest()
//                                     │                 │        └──────────────
//                                     └─────────────────┘
//

// DBConnection(Target) will work with Client
pub trait DBConnection {
    fn request(&self) -> String;
}

// Mysql struct
pub struct MySQL {
    mysql: String,
}
impl MySQL {
    fn make_sql_request(&self) -> String {
        format!("mysql://{}", self.mysql)
    }
}
impl DBConnection for MySQL {
    fn request(&self) -> String {
        self.make_sql_request()
    }
}

// PostgreSQL struct
pub struct PostgreSQL {
    postgresql: String,
}
impl PostgreSQL {
    fn make_postgresql_request(&self) -> String {
        format!("postgresql://{}", self.postgresql)
    }
}
impl DBConnection for PostgreSQL {
    fn request(&self) -> String {
        self.make_postgresql_request()
    }
}

// Composite style
// ┌─────────────┐         ┌───────────────┐               ┌──────────────────┐
// │  Client     ├─────────►  Target       │               │                  │
// └─────────────┘         ┌───────────────┤               │   Adaptee        │
//                         │               │               ├──────────────────┤
//                         │   Request()   │               │                  │
//                         └─────────────▲─┘               │  SpecificRequest()
//                                       │                 └─────────▲─────────
//                                     ┌─┴───────────────┐           │
//                                     │   Adapter       ├───────────┘
//                                     ├─────────────────┤
//                                     │                 │        ┌──────────────
//                                     │   Request()     ├────────► adaptee-> SpecificRequest()
//                                     │                 │        └──────────────
//                                     └─────────────────┘

pub struct PostgresAdapter {
    pub adaptee: PostgreSQL,
}
impl DBConnection for PostgresAdapter {
    fn request(&self) -> String {
        self.adaptee.make_postgresql_request()
    }
}

pub struct MysqlAdapter {
    pub adaptee: MySQL,
}
impl DBConnection for MysqlAdapter {
    fn request(&self) -> String {
        self.adaptee.make_sql_request()
    }
}

//Client only work with DBConnection
fn make_db_request<T: DBConnection>(db: &T) -> String {
    db.request()
}

pub fn demo_adapter() {
    let postgresql = PostgreSQL {
        postgresql: "localhost".into(),
    };
    let mysql = MySQL {
        mysql: "localhost".into(),
    };
    make_db_request(&postgresql);
    make_db_request(&mysql);

    let mysql_adapter = MysqlAdapter { adaptee: mysql };
    let postgres_adapter = PostgresAdapter {
        adaptee: postgresql,
    };
    make_db_request(&mysql_adapter);
    make_db_request(&postgres_adapter);
}
