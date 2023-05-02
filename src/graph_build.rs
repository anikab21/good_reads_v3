use petgraph::{prelude::*, visit::IntoNodeIdentifiers};
use polars::{prelude::*};

pub fn build_the_graph<'a>(
    readers_df: &'a DataFrame,
    the_graph: &mut GraphMap<i64, usize, Undirected>,
) {

    let books_read_df = readers_df
        .clone()
        .lazy()
        .groupby(["user_id"])
        .agg([col("book_id").list()])
        .select([col("book_id")])
        .collect()
        .unwrap();

    let books_read = books_read_df.column("book_id").unwrap();

    for books_read_by_user in books_read.iter() {
        match books_read_by_user {
            AnyValue::List(users_booklist) => {
                process_users_booklist(&users_booklist, the_graph);
            }
            _ => (),
        }
    }

}

fn process_users_booklist(
    users_booklist: &Series,
    the_graph: &mut GraphMap<i64, usize, Undirected>,
) {
    for booklist in users_booklist.iter() {
        match booklist {
            AnyValue::List(books) => {
                add_related_books(&books, the_graph);
            }
            _ => {}
        }
    }
}

fn add_related_books(books: &Series, the_graph: &mut GraphMap<i64, usize, Undirected>) {

    let mut book_id_vec: Vec<i64> = Vec::new();
    for book in books.iter() 
    {
        match book {
            AnyValue::Int64(book_id) => {
                book_id_vec.push(book_id);
            }
            _ => {}
        }
    }
    for (i, _book_id) in book_id_vec.iter().enumerate() {
        if i < book_id_vec.len() {
            for j in i + 1..book_id_vec.len() {
                match the_graph.add_edge(book_id_vec[i], book_id_vec[j], 1) {
                    Some(old_weight) => {
                        the_graph.add_edge(book_id_vec[i], book_id_vec[j], old_weight + 1);
                    }
                    None => (),
                }
            }
        }
    }
}

pub fn collect_stats(books_df: &DataFrame, the_graph: &mut GraphMap<i64, usize, Undirected>) {

    let nodes = the_graph.node_identifiers().collect::<Vec<i64>>();
    let node_series :Series = nodes.iter().collect();
    let node_df = DataFrame::new(vec![node_series]).unwrap();

    let df = node_df
        .join(books_df, [""], ["book_id"], JoinType::Inner, None)
        .unwrap();

    let df2 = df.clone()
    .lazy()
    .groupby([col("genre")])
    .agg([count()])
    .sort(
        "count",
        SortOptions {
            descending: false,
            nulls_last: true,
            multithreaded: true,
        })
    .collect();

    println!("{}", df2.unwrap());

}
