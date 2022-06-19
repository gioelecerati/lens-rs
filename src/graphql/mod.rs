pub struct QVar {
    pub name: String,
    pub value: String,
}

pub mod queries;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    pub query: String,
}

pub fn parse(query: String, variables: Vec<QVar>) -> Query {
    let mut q = query;
    for v in variables {
        q = q.replace(&format!("%%{}%%", v.name), &v.value);
    }
    Query { query: q }
}
