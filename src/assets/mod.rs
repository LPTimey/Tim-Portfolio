use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, LazyLock, RwLock};

pub mod img;
pub mod stylesheet;

static SEEN_ASSETS: LazyLock<RwLock<HashMap<String, (Arc<dyn Asset>, bool)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(self) fn register_used(asset: &Arc<dyn Asset>) {
    let mut used = SEEN_ASSETS.write().unwrap();
    used.entry(asset.key())
        .and_modify(|(_, used)| *used = true)
        .or_insert_with(|| (asset.clone(), true));
}
pub(self) fn register_seen_or_get(asset: Arc<dyn Asset>) -> Arc<dyn Asset> {
    let mut used = SEEN_ASSETS.write().unwrap();
    used.entry(asset.key())
        .or_insert_with(|| (asset.clone(), false))
        .0
        .clone()
}

pub fn used_assets() -> Vec<Arc<dyn Asset>> {
    SEEN_ASSETS
        .read()
        .unwrap()
        .values()
        .cloned()
        // .map(|arc|{dbg!(arc)})
        .flat_map(|(arc, used)| if used { Some(arc) } else { None })
        .collect()
}

pub trait Asset: Any + Send + Sync + std::fmt::Debug {
    fn key(&self) -> String;
    fn process(self: Arc<Self>, prefix: &Path, overwrite: bool) -> Result<(), Box<dyn Error>>;
}
