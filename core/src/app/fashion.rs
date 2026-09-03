use std::{iter, sync::Arc};

use crate::{
    domain::fashion::Fashion,
    ports::repositories::{self, FashionError, FashionRepository},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The repository returned a fashion template without id")] // should not be possible
    MissingFashionId,
    #[error(transparent)]
    Repository(#[from] repositories::Error<FashionError>),
}

pub struct Service<R>
where
    R: FashionRepository,
{
    fashion_repo: Arc<R>,
}

impl<R> Service<R>
where
    R: FashionRepository,
{
    pub fn new(fashion_repo: Arc<R>) -> Self {
        Self { fashion_repo }
    }

    pub async fn create(&self, fashion: &Fashion) -> Result<Fashion> {
        let mut created = self.fashion_repo.insert_fashion(fashion).await?;
        let id = created.id.ok_or(Error::MissingFashionId)?;
        self.fashion_repo
            .ensure_fashion_tags(iter::once(&id), &fashion.tags)
            .await?;
        created.tags.extend_from_slice(&fashion.tags);
        Ok(created)
    }
}
