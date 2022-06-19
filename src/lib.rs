pub mod graphql;
pub mod lens;
pub mod methods;
pub mod tests;

#[derive(Debug, Clone)]
pub enum Chain {
    Polygon,
}

#[derive(Debug, Clone)]
pub enum Net {
    Mumbai,
    Main,
}

