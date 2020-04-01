imports!();

use crate::client::GetQueryBuilder;

new_type!(
    Issues
);

from!(
    @GetQueryBuilder
        -> Issues = "issues"
);

impl_macro!(
    @Issues
        |
);

exec!(Issues);
