use std::fs;
use std::io::Error;

use super::full::Full;

/// Load a map from a file
/// 
/// # Parameters
/// `path` - The path of the map file
/// 
/// # Returns
/// Ok<Map> if the map could be loaded, Err<Error> containing an IO error otherwise
pub fn load_map(path: String) -> Result<Full, Error> {
    let file_str = &fs::read_to_string(path)?;

    let deserialized: Full = serde_json::from_str(file_str)?;

    Ok(deserialized)
}

/// Save a map from to file
/// 
/// # Parameters
/// * `path` - The path of the map file
/// * `map` - The map to be saved
/// 
/// # Returns
/// Ok<())> if the map could be saved, Err<Error> containing an IO error otherwise
pub fn save_map(path: String, map: &Full) -> Result<(), Error> {
    let serialized = serde_json::to_string(map)?;

    fs::write(path, serialized)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::*;

    #[test]
    fn save_load_map() {
        // Create new full map and related components
        let island_tile = IslandTile::new(IslandTileInfo::CaveOfShadows);

        let full = Full::new(Vec2::from_values(10, 10), Some(island_tile));

        // Save and load map
        save_map(String::from("test_save_load_map.maptmp"), &full).expect("Could not save the map");
        
        let full_loaded = load_map(String::from("test_save_load_map.maptmp")).expect("Could not load the map");

        // Make sure that original map and saved/loaded map are the same
        assert_eq!(full, full_loaded);
    }
}