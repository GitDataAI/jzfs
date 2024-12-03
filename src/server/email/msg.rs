use lettre::message::Mailbox;

#[derive(Clone)]
pub struct EmailMSG{
    pub from: Mailbox,
    pub reply: Mailbox,
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}