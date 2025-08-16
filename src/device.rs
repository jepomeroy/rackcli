pub trait Device {
    fn disable(&self) -> std::io::Result<()>;
    fn enable(&self) -> std::io::Result<()>;
    fn status(&self) -> std::io::Result<()>;
    fn update(&mut self);
}
