pub mod dummy;
pub mod factory;

pub use dummy::DummyBackend;

// # Define the trait that backends must implement
#[async_trait::async_trait]
pub trait AlertBackend: Send + Sync {
    async fn run(&mut self);
}
