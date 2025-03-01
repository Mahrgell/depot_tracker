pub trait FormattedProperty<Target> {
    fn header(&self) -> String;
    fn format_data(&self, t: &Target) -> String;
}
