fn main() {
    if let Err(e) = inner_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn inner_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

// #[derive(Copy, Clone, Debug)]
// enum GetOneError {
//     None(&'static str),
//     Multiple(&'static str),
// }

// impl std::fmt::Display for GetOneError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::None(context) => write!(f, "One feature required but none found for: {context}"),
//             Self::Multiple(context) => {
//                 write!(
//                     f,
//                     "One feature required but more than one found for: {context}"
//                 )
//             }
//         }
//     }
// }

// impl std::error::Error for GetOneError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }
// }

// trait IteratorExt: Iterator {
//     fn get_one(self, context: &'static str) -> Result<Self::Item, GetOneError>;
// }

// impl<T: Iterator> IteratorExt for T {
//     fn get_one(mut self, context: &'static str) -> Result<Self::Item, GetOneError> {
//         match self.next() {
//             None => Err(GetOneError::None(context)),
//             Some(res) => match self.next() {
//                 Some(_) => Err(GetOneError::Multiple(context)),
//                 None => Ok(res),
//             },
//         }
//     }
// }
