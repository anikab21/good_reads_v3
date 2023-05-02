use good_reads_v3::data_load;
use good_reads_v3::graph_analyze;
use good_reads_v3::graph_build;
use good_reads_v3::ui;
use petgraph::prelude::*;


fn main() {
    //Give intro to user
    ui::welcome();
    ui::separator();
    ui::confirm_exit("Shall we proceed with dataset selection?");
    ui::separator();

    //User's choice of dataset
    let datasets = ui::select_dataset();
    ui::separator();

    //Load books data
    let books_df = data_load::load_books_data(&datasets);
    ui::metadata(&books_df, "books");

    //Load reviews data
    let reviews_df = data_load::load_reviews_data(&datasets, &books_df);
    ui::metadata(&reviews_df, "reviews");

    //Confirmation to build the graph
    ui::separator();
    ui::confirm_exit("Shall we proceed with the graph build?");
    let mut the_graph: GraphMap<i64, usize, Undirected> = UnGraphMap::new();

    //Populate the graph with data
    graph_build::build_the_graph(&reviews_df, &mut the_graph);

    ui::separator();
    ui::graph_metadata(&the_graph);

    ui::separator();
    ui::msg(&String::from("The graph stats are:"));
    graph_build::collect_stats(&books_df, &mut the_graph);
    ui::separator();

    // let mut f = File::create("the_graph.dot").unwrap();
    // let output = format!("{}", Dot::new(&the_graph));
    // f.write_all(&output.as_bytes()).unwrap();

    ui::confirm_exit("Shall we proceed with the densest subgraph analysis?");
    ui::separator();

    let dsg_stats = graph_analyze::find_densest_subgraph(&mut the_graph, &books_df);
    // let mut f = File::create("the_subgraph.dot").unwrap();
    // let output = format!("{}", Dot::new(&dsg));
    // f.write_all(&output.as_bytes()).unwrap();

    ui::separator();
    ui::msg(&String::from("THE DENSEST SUBGRAPH IS:"));
    println!("{}", dsg_stats);
}
