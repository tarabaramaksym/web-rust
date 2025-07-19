pub mod about {
    pub mod index;
    pub use index::AboutController;
}

pub mod index {
    pub mod index;
    pub use index::HomeController;
}

pub mod not_found {
    #[path = "../404/404.rs"]
    pub mod controller;
    pub use controller::NotFoundController;
}