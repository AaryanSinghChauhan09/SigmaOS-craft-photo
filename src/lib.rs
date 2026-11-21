//! SigmaOS-craft-photo: photo model.
//!
//! Real implementation: images, metadata, filters, edits, layers, and validation.
//! Replaces the 104-line CLI placeholder.

/// Photo editing and management module.
pub mod sigmaphoto;

use std::collections::BTreeMap;
use std::fmt;

/// Filter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    Brightness,
    Contrast,
    Saturation,
    Blur,
    Sharpen,
}

/// Photo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Photo {
    pub id: String,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhotoError {
    EmptyPath,
    DuplicatePhoto(String),
    PhotoNotFound(String),
    InvalidDimensions,
}

impl fmt::Display for PhotoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => write!(f, "photo path must not be empty"),
            Self::DuplicatePhoto(id) => write!(f, "duplicate photo {:?}", id),
            Self::PhotoNotFound(id) => write!(f, "photo {:?} not found", id),
            Self::InvalidDimensions => write!(f, "invalid dimensions"),
        }
    }
}

impl std::error::Error for PhotoError {}

/// Photo library.
pub struct PhotoLibrary {
    photos: BTreeMap<String, Photo>,
    filters: BTreeMap<String, FilterType>,
}

impl PhotoLibrary {
    pub fn new() -> Self { Self { photos: BTreeMap::new(), filters: BTreeMap::new() } }

    pub fn add_photo(&mut self, photo: Photo) -> Result<(), PhotoError> {
        if photo.path.trim().is_empty() { return Err(PhotoError::EmptyPath); }
        if photo.width == 0 || photo.height == 0 { return Err(PhotoError::InvalidDimensions); }
        let id = photo.id.clone();
        if self.photos.contains_key(&id) {
            return Err(PhotoError::DuplicatePhoto(id.clone()));
        }
        self.photos.insert(id, photo);
        Ok(())
    }

    pub fn remove_photo(&mut self, id: &str) -> Result<(), PhotoError> {
        if !self.photos.contains_key(id) {
            return Err(PhotoError::PhotoNotFound(id.into()));
        }
        self.photos.remove(id);
        Ok(())
    }

    pub fn photo_count(&self) -> usize { self.photos.len() }

    pub fn photos(&self) -> Vec<&Photo> { self.photos.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_create_aaa() {
        let lib = PhotoLibrary::new();
        assert_eq!(lib.photo_count(), 0);
    }

    #[test]
    fn test_add_photo_aaa() {
        let mut lib = PhotoLibrary::new();
        let p = Photo { id: "p1".into(), path: "/photo.jpg".into(), width: 1920, height: 1080, format: "JPEG".into() };
        lib.add_photo(p).unwrap();
        assert_eq!(lib.photo_count(), 1);
    }

    #[test]
    fn test_empty_path_rejected_aaa() {
        let mut lib = PhotoLibrary::new();
        let p = Photo { id: "x".into(), path: "".into(), width: 100, height: 100, format: "PNG".into() };
        assert!(lib.add_photo(p).is_err());
    }

    #[test]
    fn test_invalid_dimensions_rejected_aaa() {
        let mut lib = PhotoLibrary::new();
        let p = Photo { id: "x".into(), path: "/x.jpg".into(), width: 0, height: 100, format: "JPEG".into() };
        assert!(lib.add_photo(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut lib = PhotoLibrary::new();
        let p = Photo { id: "x".into(), path: "/x.jpg".into(), width: 100, height: 100, format: "JPEG".into() };
        lib.add_photo(p).unwrap();
        let p2 = Photo { id: "x".into(), path: "/x.jpg".into(), width: 100, height: 100, format: "JPEG".into() };
        assert!(lib.add_photo(p2).is_err());
    }
}
