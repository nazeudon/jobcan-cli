use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PageTransitionError();

impl fmt::Display for PageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ページ遷移に失敗しました")
    }
}

impl Error for PageTransitionError {}
