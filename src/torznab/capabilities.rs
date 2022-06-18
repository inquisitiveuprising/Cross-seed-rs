use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportedParam {
    Query,
    Season,
    Episode,
    IMDB,
    TMDB,
    TVDB,
}

impl From<String> for SupportedParam {
    fn from(s: String) -> Self {
        match s.as_str() {
            "q" => SupportedParam::Query,
            "season" => SupportedParam::Season,
            "ep" => SupportedParam::Episode,
            "imdbid" => SupportedParam::IMDB,
            "tmdbid" => SupportedParam::TMDB,
            "tvdbid" => SupportedParam::TVDB,
            _ => panic!("Unsupported param: {}", s),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Deserialize)]
pub enum SearchCapability {
    #[serde(rename = "search")]
    Search,

    #[serde(rename = "tv-search")]
    TV,
    
    #[serde(rename = "movie-search")]
    Movie,
    
    #[serde(rename = "music-search")]
    Music,
    
    #[serde(rename = "audio-search")]
    Audio,
    
    #[serde(rename = "book-search")]
    Book
}

impl From<String> for SearchCapability {
    fn from(s: String) -> Self {
        match s.as_str() {
            "search" => SearchCapability::Search,
            "tv-search" => SearchCapability::TV,
            "movie-search" => SearchCapability::Movie,
            "music-search" => SearchCapability::Music,
            "audio-search" => SearchCapability::Audio,
            "book" => SearchCapability::Book,
            _ => panic!("Unsupported param: {}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchingCapabilities {
    supported_functions: HashMap<SearchCapability, Vec<SupportedParam>>,
}

impl SearchingCapabilities {
    pub fn new(supported_functions: HashMap<SearchCapability, Vec<SupportedParam>>) -> SearchingCapabilities {
        SearchingCapabilities {
            supported_functions,
        }
    }

    /// Returns true if the search type is supported.
    pub fn does_support_search(&self, search_capability: SearchCapability) -> bool {
        self.supported_functions.contains_key(&search_capability)
    }

    /// Returns true if a search type supports a specific parameter.
    pub fn does_search_support_param(&self, capability: SearchCapability, param: SupportedParam) -> bool {
        match self.supported_functions.get(&capability) {
            Some(params) => params.contains(&param),
            None => false,
        }
    }
}

impl Default for SearchingCapabilities {
    fn default() -> Self {
        SearchingCapabilities {
            supported_functions: HashMap::new(),
        }
    }
}

impl<'de> Deserialize<'de> for SearchingCapabilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw: HashMap<SearchCapability, HashMap<String, String>> = Deserialize::deserialize(deserializer).unwrap();
        let mut functions: HashMap<SearchCapability, Vec<SupportedParam>> = HashMap::new();

        for (key, value) in raw.iter() {
            let mut supported_params = Vec::new();

            let available_str: String = value.get("available").map(String::to_owned).unwrap_or_default();
            let available = available_str == "yes";

            if available {
                if let Some(params) = value.get("supportedParams") {
                    for param in params.split(',') {
                        supported_params.push(param.to_string().into());
                    }
                }

                functions.insert(key.clone(), supported_params);
            }
        }

        Ok(SearchingCapabilities {
            supported_functions: functions,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    #[serde(with = "serde_with::rust::display_fromstr")]
    id: u32,
    name: String,

    #[serde(rename = "subcat")]
    sub_categories: Option<Vec<Category>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Categories {
    #[serde(rename = "category")]
    pub categories: Vec<Category>,
}

impl Categories {
    pub fn new(categories: Vec<Category>) -> Self {
        Categories { categories }
    }
}

impl Default for Categories {
    fn default() -> Self {
        Categories {
            categories: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Capabilities {
    pub categories: Categories,
    
    #[serde(rename = "searching")]
    pub searching_capabilities: SearchingCapabilities,
}

impl Default for Capabilities {
    fn default() -> Self {
        Capabilities {
            categories: Categories::default(),
            searching_capabilities: SearchingCapabilities::default(),
        }
    }
}