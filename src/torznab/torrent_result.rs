use rss::Item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultError {
    MissingTitle,
    MissingLink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorrentResult<'a> {
    name: &'a str,
    link: &'a str,
    /* size: u64,
    categories: Vec<u32>, */
}

impl<'a> TorrentResult<'a> {
    pub fn from_item(item: &'a Item) -> Result<Self, ResultError> {
        let name = item.title().ok_or(ResultError::MissingTitle)?;
        let link = item.link().ok_or(ResultError::MissingLink)?;
        /* let size = item.enclosure().map(|e| e.length().parse::<u64>());
        let categories = item.categories().ok_or(ResultError::MissingTitle)?; */

        Ok(TorrentResult {
            name,
            link,
            /* size,
            categories, */
        })
    }
}

/* impl<'a> From<Item> for TorrentResult<'a> {
    fn from(item: Item) -> Self {
        TorrentResult {
            name: item.title().unwrap(),
            link: item.link().unwrap(),
            size: item.size().unwrap(),
            categories: item.categories().unwrap(),
        }
    }
} */