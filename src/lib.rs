#[derive(Debug, Clone)]
pub struct PhotoAdjustments { pub exposure: f32, pub contrast: f32 }
impl Default for PhotoAdjustments {
    fn default() -> Self { Self { exposure: 0.0, contrast: 1.0 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_photo() {
        let adj = PhotoAdjustments::default();
        assert_eq!(adj.contrast, 1.0);
    }
}
