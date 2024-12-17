use crate::metadata::model::users;


pub mod groups_users;
pub mod groups_invites;
pub mod groups_labels;
pub mod groups_data;
pub use {
    users::users as groups,
    users::users::Model as Groups,
    groups_users::Model as GroupsUsers,
    groups_invites::Model as GroupsInvites,
    groups_labels::Model as GroupsLabels,
};
