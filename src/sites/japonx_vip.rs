mod urls { pub const LATEST_POSTS_PAGE: &'static str = "https://www.japonx.vip/portal/index/search/new/1.html?page="; }

// --- std ---
use std::{
    collections::HashSet,
    fmt::{Formatter, Display, self},
};
// --- external ---
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
// --- custom ---
use super::{CRAWLER, Site};
