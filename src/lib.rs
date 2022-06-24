pub mod crypto;
pub mod graphql;
pub mod lens;
pub mod methods;
pub mod tests;
pub mod utils;

#[derive(Debug, Clone)]
pub enum Chain {
    Polygon,
}

#[derive(Debug, Clone)]
pub enum Net {
    Mumbai,
    Main,
}
