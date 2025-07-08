mod bucket;
mod error;
mod sinker;

pub use error::{Error, Result};

pub use sinker::{Buckets, Sinker};

pub use bucket::Bucket;
