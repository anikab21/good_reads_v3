use crate::common::{DataSet, Genre};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

//Input, MultiSelect,  Sort};
use super::common;
use petgraph::prelude::*;
use polars::prelude::*;
use std::process;

pub fn confirm_exit(msg: &str) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()
        .unwrap()
    {
        println!("Great!");
    } else {
        println!("Good bye!");
        process::exit(0);
    }
}

pub fn plain_confirm(msg: &String) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()
        .unwrap()
}

pub fn metadata(df: &DataFrame, msg: &str) {
    println!(" Loaded {} {}", df.column("book_id").unwrap().len(), msg);
}

pub fn graph_metadata(graph: &GraphMap<i64, usize, Undirected>) {
    println!(" Graph built with:");
    println!("    Vertices: {}", graph.node_count());
    println!("    Edges: {}", graph.edge_count());
}

pub fn welcome() {
    let mut welcome_msg: String = String::new();

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    println!("      Hi! - I am Good Reads data analyzer   \n");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

    welcome_msg += " I have collected data on book reviews from goodreads.com\n";
    welcome_msg += " I can analyze this data for you using graph algorithms\n";

    welcome_msg += "\n Following master data is available\n";
    welcome_msg += "    Genre: CHILDREN, Books: 999\n";
    welcome_msg += "    Genre: COMICS, Books: 999\n";
    welcome_msg += "    Genre: MYSTERY, Books: 999\n";
    welcome_msg += "    Genre: POETRY, Books: 999\n";
    welcome_msg += "\n You can choose your own subset for data analysis\n";

    println!("{}", welcome_msg);
}

pub fn select_dataset() -> Vec<DataSet> {
    let mut datasets: Vec<DataSet> = Vec::new();

    loop {
        if datasets.is_empty() {
            match select_genre() {
                Some(ds) => {
                    datasets.push(ds);
                }
                None => {}
            }
        } else {
            disp_datasets(&datasets);
            let more_books = plain_confirm(&String::from("Add more genres / books to your dataset?"));
            if more_books {
                match select_genre() {
                    Some(ds) => {
                        datasets.push(ds);
                    }
                    None => {}
                }
            } else {
                break;
            }
        }
    }
    datasets
}

fn select_genre() -> Option<DataSet> {
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    let items = [
        common::Genre::CHILDREN,
        common::Genre::COMICS,
        common::Genre::MYSTERY,
        common::Genre::POETRY,
    ];

    let genre_selection: usize = Select::with_theme(&theme)
        .with_prompt("PLEASE SELECT A GENRE TO ADD TO YOUR DATASET: \n")
        .items(&items)
        .default(0)
        .interact_on(&term)
        .unwrap();

    let mut books: usize;

    loop {
        books = Input::with_theme(&theme)
            .with_prompt("No of books from this genre: (<1000) ")
            .interact_on(&term)
            .unwrap();

        if books <1000{
            break;
        }
    }

    match genre_selection {
        0 => {
            let genre = Genre::CHILDREN;
            let ds: DataSet = DataSet { genre, books };
            return Some(ds);
        }
        1 => {
            let genre = Genre::COMICS;
            let ds: DataSet = DataSet { genre, books };
            return Some(ds);
        }
        2 => {
            let genre = Genre::MYSTERY;
            let ds: DataSet = DataSet { genre, books };
            return Some(ds);
        }
        3 => {
            let genre = Genre::POETRY;
            let ds: DataSet = DataSet { genre, books };
            return Some(ds);
        }
        _ => None,
    }
}

fn disp_datasets(datasets: &Vec<DataSet>) {
    blank_line();
    separator();
    let mut total_books: usize = 0;
    println!("YOUR DATASET:");
    println!("-------------");
    for ds in datasets {
        println!("{}", ds);
        total_books += ds.books;
    }
    println!("TOTAL BOOKS: {} ", total_books);
    separator();
    blank_line();
}

pub fn msg(msg: &String) {
    println!("{msg}");
}

pub fn separator() {
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}

pub fn blank_line() {
    println!("");
}
