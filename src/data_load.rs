use super::common;
use polars::prelude::*;


//Generic Data loader - loads data from given CSV file path
//Specific data loaders will provide specific path from which to load data
fn load_data(path: &String) -> DataFrame {
    let df = CsvReader::from_path(path)
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    df
}

//Specific data loader for books data
pub fn load_books_data(datasets: &Vec<common::DataSet>) -> DataFrame {

    let mut books_df : DataFrame = DataFrame::default();

    for dataset in datasets {
        let data_path;
        match dataset.genre {
            common::Genre::CHILDREN => {
                data_path = String::from("data/CHILDREN_BOOKS.csv");
            }
            common::Genre::COMICS => {
                data_path = String::from("data/COMICS_BOOKS.csv");
            }
            common::Genre::MYSTERY => {
                data_path = String::from("data/MYSTERY_BOOKS.csv");
            }
            common::Genre::POETRY => {
                data_path = String::from("data/POETRY_BOOKS.csv");
            }
        }

        let tdf = load_data(&data_path);

        let bdf = tdf.sample_n(dataset.books, false, true, Some(0)).unwrap();

        books_df = books_df.vstack(&bdf.clone()).unwrap();
    }
    books_df.rechunk();

    println!("{}", books_df.clone());

    books_df

}

pub fn load_reviews_data(datasets: &Vec<common::DataSet>, books_df: &DataFrame) -> DataFrame {
    let mut reviews_df : DataFrame = DataFrame::default();

    for dataset in datasets {
        let data_path : String;
        match dataset.genre {
            common::Genre::CHILDREN => {
                data_path = String::from("data/CHILDREN_REVIEWS.csv");
            }
            common::Genre::COMICS => {
                data_path = String::from("data/COMICS_REVIEWS.csv");
            }
            common::Genre::MYSTERY => {
                data_path = String::from("data/MYSTERY_REVIEWS.csv");
            }
            common::Genre::POETRY => {
                data_path = String::from("data/POETRY_REVIEWS.csv");
            }
        }

        let tdf = load_data(&data_path);

        let bdf = tdf
            .join(&books_df, ["book_id"], ["book_id"], JoinType::Inner, None)
        .unwrap();

        let cdf = bdf.clone()
            .lazy()
            .select([col("book_id"), col("user_id")])
            .collect()
            .unwrap();

        reviews_df = reviews_df.vstack(&cdf.clone()).unwrap();
    }
    reviews_df.rechunk();

    println!("{}", reviews_df.clone());

    reviews_df
}


