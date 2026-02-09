//! Mock endpoint registration for httpbin-proto-mock
//! Using orb-mockhttp with full handler integration

mod adapter;
pub mod any;
pub mod h1;
pub mod h2;
pub mod h3;

pub use any::register_any_protocol_mocks;
pub use h1::register_h1_mocks;
pub use h2::register_h2_mocks;
pub use h3::register_h3_mocks;
