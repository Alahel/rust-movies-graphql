mod models;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use models::QueryRoot;
use scooby::postgres::select;
use slab::Slab;
use std::collections::HashMap;
pub type RootSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
use serde::Deserialize;
use tokio_postgres::Client;

pub struct IMovie {
    id: &'static str,
    title: &'static str,
}
#[derive(Clone, Debug, Deserialize)]
pub struct DeserMovie {
    id: String,
    title: String,
}

pub struct IMovies {
    client: Client,
    items: Slab<IMovie>,
    items_by_id: HashMap<&'static str, usize>,
}

impl IMovies {
    #[allow(clippy::new_without_default)]
    pub fn new(client: Client) -> Self {
        let mut items: Slab<IMovie> = Slab::new();

        items.insert(IMovie {
            id: "1",
            title: "Movie 1",
        });
        items.insert(IMovie {
            id: "2",
            title: "Movie 2",
        });

        let items_by_id = items.iter().map(|(idx, ch)| (ch.id, idx)).collect();
        Self {
            client,
            items,
            items_by_id,
        }
    }

    pub fn item(&self, id: &str) -> Option<&IMovie> {
        self.items_by_id
            .get(id)
            .copied()
            .map(|idx: usize| self.items.get(idx).unwrap())
    }

    pub async fn items(&self) -> Vec<&IMovie> {
        let query = select(("id", "title")).from("movie.movie").limit(10);
        let rows = self.client.query(&query.to_string(), &[]).await;

        let mut results: Vec<&IMovie> = rows
            .into_iter()
            .map(|row| {
                let r = row.get(0);
                let rr = r.expect("eee");
                // let id = String::from(rr.get("id"));
                // let title: String = rr.get(0);
                // print!("val: {}", title);
                &IMovie {
                    id: "1",
                    title: "sddsdsd",
                }
            })
            .collect();

        results.push(&IMovie {
            id: "2",
            title: "Movie 2",
        });
        results
    }
}
