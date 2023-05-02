use super::ui;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;
use polars::prelude::*;

fn graph_density(any_graph: &GraphMap<i64, usize, Undirected>) -> f64 {
    let edge_count = any_graph.edge_count() as f64;
    let node_count = any_graph.node_count() as f64;
    edge_count / node_count
}

fn min_degree_node(the_graph: &GraphMap<i64, usize, Undirected>) -> Option<i64> {
    let mut min_degree = the_graph.node_count();
    let mut min_degree_node: Option<i64> = None;

    for node_id in the_graph.node_identifiers() {

        // let mut node_degree = 0;
        // for neighbor in the_graph.neighbors(node_id)
        // {
        //     let edge_weight = the_graph.edge_weight(node_id, neighbor);
        //     match edge_weight {
        //         Some(weight) => {node_degree += weight;},
        //         None => {}
        //     }
        // }

        let node_degree = the_graph.neighbors(node_id).count();

        if node_degree < min_degree {
            min_degree_node = Some(node_id);
            min_degree = node_degree;
        }
    }
    min_degree_node
}

pub fn find_densest_subgraph(
    the_graph: &mut GraphMap<i64, usize, Undirected>,
    books_df: &DataFrame,
) -> DataFrame {
    let mut iterative_graph_stats: Vec<DataFrame> = Vec::new();
    let mut iterative_densities: Vec<f64> = Vec::new();

    let mut iterative_density = graph_density(&the_graph);

    let nodes_count = the_graph.node_count();

    let mut start_msg: String = String::from("I will be performing ");
    start_msg.push_str(&(nodes_count - 1).to_string());
    start_msg.push_str(" iterations \n");
    start_msg.push_str("Would you like to see result of each iteration?");
    let mut deep_dive: bool = ui::plain_confirm(&start_msg);
    ui::separator();

    let mut stats_df = collect_stats(books_df, &the_graph);

    for i in 1..nodes_count {

     
        if deep_dive {
            let mut msg = String::from("\nResult from iteration #");
            msg.push_str(&(i - 1).to_string());
            ui::msg(&msg);
            println!("{}", &stats_df);
        }

        if i > 1 && deep_dive {
            deep_dive = ui::plain_confirm(&String::from(
                "\n Continue to deep dive into each iteration result?",
            ));
        }

        

        if deep_dive {
            ui::separator();
            let mut msg = String::from("Iteration #");
            msg.push_str(&i.to_string());
            msg.push_str(" of ");
            msg.push_str(&(nodes_count - 1).to_string());
            ui::msg(&msg);
        }

        iterative_density = graph_density(&the_graph);

        iterative_graph_stats.push(stats_df);
        iterative_densities.push(iterative_density);

        let min_degree_node = min_degree_node(&the_graph);
        match min_degree_node {
            Some(node_id) => {
                the_graph.remove_node(node_id);
            }
            None => {}
        }
        stats_df = collect_stats(books_df, &the_graph);
    }

    let mut max_idx: usize = 0;

    for (i, an_iterative_density) in iterative_densities.iter().enumerate() {
        if *an_iterative_density > iterative_density {
            iterative_density = *an_iterative_density;
            max_idx = i;
        }
    }

    let densest_graph_stats = iterative_graph_stats[max_idx].clone();
    densest_graph_stats
}

fn collect_stats(books_df: &DataFrame, the_graph: &GraphMap<i64, usize, Undirected>) -> DataFrame {
    let nodes = the_graph.node_identifiers().collect::<Vec<i64>>();
    let node_series: Series = nodes.iter().collect();
    let node_df = DataFrame::new(vec![node_series]).unwrap();

    let df = node_df
        .join(books_df, [""], ["book_id"], JoinType::Inner, None)
        .unwrap();

    let df2 = df
        .clone()
        .lazy()
        .groupby([col("genre")])
        .agg([count()])
        .sort(
            "count",
            SortOptions {
                descending: false,
                nulls_last: true,
                multithreaded: true,
            },
        )
        .collect().unwrap();

    df2

}
