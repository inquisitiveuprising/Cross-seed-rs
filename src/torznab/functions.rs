use super::search_parameters::*;

#[derive(Debug)]
pub enum SearchFunction {
    /// Free text search query.
    Search,
    /// Search query with tv specific query params and filtering.
    TVSearch(TVSearchParameters),
    /// Search query with movie specific query params and filtering.
    MovieSearch(MovieSearchParameters),

    // TODO

    /// Search query with music specific query params and filtering.
    MusicSearch,
    /// Search query with book specific query params and filtering.
    BookSearch,
}

impl SearchFunction {
    pub fn to_function_str(&self) -> &str {
        match self {
            SearchFunction::Search => "search",
            SearchFunction::TVSearch(_) => "tvsearch",
            SearchFunction::MovieSearch(_) => "movie",
            SearchFunction::MusicSearch => "music",
            SearchFunction::BookSearch => "book",
        }
    }

    pub fn to_params(&self) -> String {
        let mut params = String::new();

        params.push_str(&format!("&t={}", self.to_function_str()));

        // Concatenate the params of the search function.
        match self {
            SearchFunction::Search => {},//params.push_str(&s),
            SearchFunction::TVSearch(p) => params.push_str(&p.to_params()),
            SearchFunction::MovieSearch(p) => params.push_str(&p.to_params()),
            _ => panic!("Not implemented!"), // TODO
        }

        params
    }
}

#[derive(Debug)]
pub enum TorznabFunction {
    /// Returns the capabilities of the api.
    Capabilities,
    
    SearchFunction(GenericSearchParameters, SearchFunction),

    // TODO

    /// (newznab) Returns all details about a particular item.
    Details,
    /// (newznab) Returns an nfo for a particular item.
    GetNFO,
    /// (newznab) Returns nzb for the specified item.
    GetNZB,
    /// (newznab) Adds item to the users cart.
    CardAdd,
    /// (newznab) Removes item from the users cart.
    CardDel,
    /// (newznab) Returns all comments known about an item.
    Comments,
    /// (newznab) Adds a comment to an item.
    CommentsAdd,
    /// (newznab) Register a new user account.
    Register,
    /// (newznab) Retrieves information about an user account.
    User
}

impl TorznabFunction {
    pub fn to_function_str(&self) -> &str {
        match self {
            TorznabFunction::Capabilities => "caps",
            TorznabFunction::SearchFunction(_, func) => func.to_function_str(),
            _ => panic!("Not implemented! ({:?})", self),
        }
    }

    pub fn to_params(&self) -> String {
        let mut params = String::new();

        // Concatenate the params of the search function.
        match self {
            TorznabFunction::SearchFunction(p, func) => {
                params.push_str(&p.to_params());
                params.push_str(&func.to_params());
            },
            _ => params.push_str(&format!("&t={}", self.to_function_str())),
        }

        params
    }
}