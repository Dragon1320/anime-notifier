use std::{error::Error, result::Result};

pub type BoxResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;
