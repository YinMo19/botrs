use serde::{Deserialize, Serialize};

/// Pagination information for list responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    /// The current page number
    pub page: u32,
    /// The number of items per page
    pub per_page: u32,
    /// The total number of items
    pub total: u32,
    /// The total number of pages
    pub total_pages: u32,
    /// Whether there is a next page
    pub has_next: bool,
    /// Whether there is a previous page
    pub has_prev: bool,
}

impl Pagination {
    /// Creates a new pagination info.
    pub fn new(page: u32, per_page: u32, total: u32) -> Self {
        let total_pages = total.div_ceil(per_page); // Ceiling division
        Self {
            page,
            per_page,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

/// Paginated list response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The list of items
    pub items: Vec<T>,
    /// Pagination information
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response.
    pub fn new(items: Vec<T>, pagination: Pagination) -> Self {
        Self { items, pagination }
    }

    /// Returns true if there are more pages.
    pub fn has_more(&self) -> bool {
        self.pagination.has_next
    }

    /// Gets the next page number if available.
    pub fn next_page(&self) -> Option<u32> {
        if self.pagination.has_next {
            Some(self.pagination.page + 1)
        } else {
            None
        }
    }

    /// Gets the previous page number if available.
    pub fn prev_page(&self) -> Option<u32> {
        if self.pagination.has_prev {
            Some(self.pagination.page - 1)
        } else {
            None
        }
    }
}
