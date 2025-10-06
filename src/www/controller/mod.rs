pub mod about {
    pub mod index;
    pub use index::AboutController;
}

pub mod index {
    pub mod index;
    pub use index::HomeController;
}

pub mod not_found {
    pub mod not_found;
    pub use not_found::NotFoundController;
}