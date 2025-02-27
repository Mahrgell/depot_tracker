pub trait Property<Target> {
    fn header(&self) -> String;
    fn format_data(&self, t: &Target) -> String;
}
