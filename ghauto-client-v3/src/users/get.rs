imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Emails
    User
);

from!(
    @GetQueryBuilder
        -> User = "user"
    @User
        -> Emails = "emails"
);

impl_macro!(
    @User
        |=> emails -> Emails
        |
);

exec!(Emails);
exec!(User);
