#[cfg(test)]
mod tests {
    #[test]
    fn test_rapidhash() {
        assert_eq!(rapidhash::v3::rapidhash_v3(b"hello"), 3327445792987248966);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_hashmap() {
        use rapidhash::{RapidHashMap, HashMapExt};

        let mut map = RapidHashMap::new();
        map.insert("key", "value1");
        assert_eq!(map.get("key"), Some(&"value1"));
        assert_eq!(map.get("na"), None);
    }

    #[cfg(feature = "rng")]
    #[test]
    fn test_rng() {
        use rapidhash::rng::RapidRng;

        let mut rng = RapidRng::new(0);
        assert_ne!(rng.next(), rng.next());
    }
}
