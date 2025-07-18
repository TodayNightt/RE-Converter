// use std::fmt::Display;
// 
// pub type Result<T> = core::result::Result<T, Error>;
// 
// #[derive(Debug)]
// pub enum Error {
//     SerdeError(String),
//     PathError,
//     IoError(String),
//     NoLastSaved,
// }
// 
// impl Display for Error {
//     // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::SerdeError(content) => f.write_str(content),
//             Error::PathError => f.write_str("PathError"),
//             Error::IoError(content) => f.write_str(content),
//             Error::NoLastSaved => f.write_str("No last saved"),
//         }
//     }
// }
// 
// impl std::error::Error for Error {}
