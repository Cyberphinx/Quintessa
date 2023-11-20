use axum::http::StatusCode;
use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PagingParams {
    pub page_number: u64,
    #[serde(default = "PagingParams::default_page_size")]
    pub page_size: u64,
}

impl Default for PagingParams {
    fn default() -> Self {
        Self {
            page_number: 0,
            page_size: 100,
        }
    }
}

impl PagingParams {
    fn default_page_size() -> u64 {
        100
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub current_page: u64,
    pub total_pages: u64,
    pub page_size: u64,
    pub total_count: u64,
}

impl<I> PaginatedResponse<I> {
    pub async fn from_paginator<C, S>(
        page_number: u64,
        page_size: u64,
        paginator: Paginator<'_, C, S>,
    ) -> Result<PaginatedResponse<S::Item>, AppError>
    where
        C: ConnectionTrait,
        S: SelectorTrait,
    {
        Ok(PaginatedResponse::<S::Item> {
            data: paginator.fetch_page(page_number).await.map_err(|error| {
                eprintln!("Error fetching page {}: {}", &page_number, error);
                AppError::new(StatusCode::BAD_REQUEST, "Error fetching page")
            })?,
            current_page: page_number,
            total_pages: paginator.num_pages().await.map_err(|error| {
                eprintln!("Error getting total number of pages: {}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Oops, something went wrong",
                )
            })?,
            page_size,
            total_count: paginator.num_items().await.map_err(|error| {
                eprintln!("Error getting total number of items: {}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Oops, something went wrong",
                )
            })?,
        })
    }
}
