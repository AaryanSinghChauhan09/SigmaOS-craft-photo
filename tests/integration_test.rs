use sigmaos_craft_photo::{Photo, PhotoLibrary};

#[test]
fn photo_library_integration_aaa() {
    let mut lib = PhotoLibrary::new();
    lib.add_photo(Photo { id: "p1".into(), path: "/photo.jpg".into(), width: 1920, height: 1080, format: "JPEG".into() }).unwrap();
    assert_eq!(lib.photo_count(), 1);
}
