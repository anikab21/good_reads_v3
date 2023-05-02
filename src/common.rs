use std::fmt;

pub enum Genre {
    CHILDREN,
    COMICS,
    POETRY,
    MYSTERY
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Genre::CHILDREN => write!(f, "CHILDREN"),
            Genre::COMICS => write!(f, "COMICS"),
            Genre::MYSTERY => write!(f, "MYSTERY"),
            Genre::POETRY => write!(f, "POETRY"),
        }
    }
}

pub struct DataSet {
    pub genre : Genre,
    pub books : usize
}


impl fmt::Display for DataSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Genre: {}, Books: {})", self.genre, self.books)
    }
}


