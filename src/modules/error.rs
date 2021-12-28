use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PageTransitionError();
#[derive(Debug)]
pub struct HomeDirectoryNotFoundError();

impl fmt::Display for PageTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ページ遷移に失敗しました")
    }
}

impl fmt::Display for HomeDirectoryNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ホームディレクトリが見つかりません")
    }
}

impl Error for PageTransitionError {}
impl Error for HomeDirectoryNotFoundError {}
