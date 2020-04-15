imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Git
    Owner
    Refs
    Repo
    Repos
);

from!(
    @PostQueryBuilder
        -> Repos = "repos"
    @Repos
        => Owner
    @Owner
        => Repo
    @Repo
        => Git
    @Git
        => Refs
);

impl_macro!(
    @Repos
        |
        |=> owner -> Owner = username_str
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> git -> Git
        |
    @Git
        |=> refs -> Refs
        |
    @Refs
        |
);

exec!(Refs);
