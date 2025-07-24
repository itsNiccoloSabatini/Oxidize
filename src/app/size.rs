#[derive(Default)]
pub struct Size {}

impl Size {
    #[expect(clippy::unused_self)]
    pub fn text_body_size(&self) -> f32 {
        14.0
    }

    #[expect(clippy::unused_self)]
    pub fn text_heading_size(&self) -> f32 {
        26.0
    }

    #[expect(clippy::unused_self)]
    pub fn space_after_heading(&self) -> f32 {
        8.0
    }
}
