mod models;

use std::collections::HashMap;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use models::QueryRoot;
use slab::Slab;
pub type RootSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct IMovie {
    id: &'static str,
    title: &'static str,
}

pub struct IMovies {
    items: Slab<IMovie>,
    items_by_id: HashMap<&'static str, usize>,
}

impl IMovies {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
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
        Self { items, items_by_id }
    }

    pub fn item(&self, id: &str) -> Option<&IMovie> {
        self.items_by_id
            .get(id)
            .copied()
            .map(|idx: usize| self.items.get(idx).unwrap())
    }

    pub fn items(&self) -> Vec<&IMovie> {
        self.items.iter().map(|(_, ch)| ch).collect()
    }
}
