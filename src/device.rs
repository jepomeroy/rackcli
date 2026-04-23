pub trait Device {
    async fn disable(&mut self) -> std::io::Result<()>;
    async fn enable(&mut self) -> std::io::Result<()>;
    async fn status(&mut self);
    fn update(&mut self);
}
