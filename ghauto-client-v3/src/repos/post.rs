imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Git
    Owner
    Pulls
    Ref
    Refs
    Repo
    Repos
    Sha
);

from!(
    @Git
        -> Refs = "refs"
    @Owner
        => Repo
    @PostQueryBuilder
        -> Repos = "repos"
    @Repo
        -> Git = "git"
        -> Pulls = "pulls"
    @Repos
        => Owner
);

impl_macro!(
    @Git
        |=> refs -> Refs
        |
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Refs
        |
    @Repo
        |=> pulls -> Pulls
        |=> git -> Git
        |
    @Repos
        |
        |=> owner -> Owner = username_str
);

exec!(Pulls);
exec!(Refs);
