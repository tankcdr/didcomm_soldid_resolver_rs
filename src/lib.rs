mod constants;
mod errors;
mod did_doc_builder;

pub mod resolver;
pub mod w3c_doc;

pub use resolver::SolResolver;
pub use w3c_doc::W3cDidDocument;
