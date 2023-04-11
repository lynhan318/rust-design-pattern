// #Intend
// Seperate the construction of a complex from its representation so that
// the same construction process can create different representations.

// #Problem
// Complex Query builder for Postgresql
// Context: a query with different select, limit,order by...

//                                             ┌───────────────────
//                                             │ QueryBuilder(table:string)
//                                             │                  │
//                                             └────────▲─────────┘
//                                                      │
//                                                      │
//                                                      │
//                                                      │
//                            ┌───────────────────────┬─┴────────────────────────────┐
//                            │                       │                              │
//         ┌──────────────────┴───┐     ┌─────────────┴────────────┐      ┌──────────┴─────────────────┐
//         │UserQueryBuilder      │     │   PostQueryBuilder       │      │   TagQueryBuilder          │
//         │                      │     │                          │      │                            │
//         │#QueryBuilder("user") │     │  #QueryBuilder("posts")  │      │   #QueryBuilder("tags")    │
//         │                      │     │                          │      │                            │
//         └───────┬──────────────┘     └────────────┬─────────────┘      │                            │
//                 │                                 │                    └───────────────┬────────────┘
//                 │                                 │                                    │
//                 │                                 │                                    │
//                 │                                 │                                    │
//                 ▼                                 ▼                                    ▼

//        select * from users...              select * from posts                   select * from tags....

pub struct QueryBuilder<'a> {
    table: &'a str,
    select: Option<&'a str>,
    limit: Option<i64>,
    offset: Option<i64>,
    order_by: Option<&'a str>,
}
impl<'a> QueryBuilder<'a> {
    pub fn new(tale: &'a str) -> Self {
        Self {
            table,
            select: None,
            limit: None,
            offset: None,
            order_by: None,
        }
    }
    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: i64) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn order_by(&mut self, order_by: &'a str) -> &mut Self {
        self.order_by = Some(order_by);
        self
    }
    pub fn select(&mut self, select: &'a str) -> &mut Self {
        self.select = Some(select);
        self
    }
    pub fn build(&self) -> String {
        let mut query_str = String::from("");
        query_str = format!(
            "SELECT {} from {}",
            query_str + self.select.unwrap_or("*"),
            self.table
        );
        if self.limit.is_some() {
            query_str = format!("{} LIMIT {}", query_str, self.limit.unwrap());
        }
        if self.offset.is_some() {
            query_str = format!("{} OFFSET {}", query_str, self.offset.unwrap());
        }
        if self.order_by.is_some() {
            query_str = format!("{} order by {}", query_str, self.order_by.unwrap());
        }
        query_str
    }
}

