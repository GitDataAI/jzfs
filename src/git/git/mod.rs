use git2::Repository;

pub mod blob;
pub mod branchs;
pub mod commits;
pub mod options;
pub mod tree;

pub struct GitLocal {
    pub repository: Repository,
}
