use cw_storage_plus::Item;
use package::Config;

pub const CONFIG: Item<Config> = Item::new("config");
