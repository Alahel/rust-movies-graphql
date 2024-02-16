#![allow(clippy::needless_lifetimes)]

use async_graphql::{
    connection::{query, Connection, Edge},
    Context, Error, Interface, Object, OutputType, Result,
};

use super::IMovies;
use crate::IMovie;

pub struct Movie<'a>(&'a IMovie);

#[Object]
impl<'a> Movie<'a> {
    async fn id(&self) -> &str {
        self.0.id
    }
    async fn title(&self) -> &str {
        self.0.title
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn movie<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the movie")] id: String,
    ) -> Option<Movie<'a>> {
        ctx.data_unchecked::<IMovies>().item(&id).map(Movie)
    }

    async fn movies<'a>(
        &self,
        ctx: &Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Movie<'a>>> {
        let movies = ctx.data_unchecked::<IMovies>().items().await;        
        query_movies(after, before, first, last, &movies, Movie).await
    }
}

#[derive(Interface)]
#[graphql(field(name = "id", ty = "&str"), field(name = "title", ty = "&str"))]
pub enum Character<'a> {
    Movie(Movie<'a>),
}

async fn query_movies<'a, F, T>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    movies: &[&'a IMovie],
    map_to: F,
) -> Result<Connection<usize, T>>
where
    F: Fn(&'a IMovie) -> T,
    T: OutputType,
{
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start: usize = 0usize;
            let mut end = movies.len();

            if let Some(after) = after {
                if after >= movies.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &movies[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < movies.len());
            connection.edges.extend(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, (map_to)(item))),
            );
            Ok::<_, Error>(connection)
        },
    )
    .await
}
