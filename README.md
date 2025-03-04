# git-author-manager

tool to make commiting as different authors (i.e., if you're peer programming on the same machine) easier.

## installation

installing with `cargo install`: `cargo install --git https://github.com/camper0008/git-author-manager`

## using

when installed, it ships a binary called `git-aum`, (git **au**thor **m**anager), which means you can run commands such as `git aum get` to get the current status

you can:
- `git aum get` to get the current config and authors available in the found config files
- `git aum set <id>` to set yourself as an added author
- `git aum add <id> <name> email>` to add an author to the config
- `git aum remove <id>` (or `git aum rm <id>`) to remove an author from the config
- `git aum doas <id> <cmd>` to run a cmd as an author, and then change back to your previous config
- `git aum add-from-git <id>` to add an author based on your current git config

additionally, there is a `-v, --verbose` flag for all of the commands which prints information such as which config file is being read from

for example, if you have a user called `tph`, and a user called `mtk` you can:
set the author to tph: `git aum set tph`
commit as mtk: `git aum doas mtk 'git commit -m "refactored board state"'`
and verify that you are still tph: `git aum get`

the software works on a repository basis, and it only writes to the git config on the `local` level, i.e. a repository's `.git/config`, so it shouldn't corrupt your global config in the event of a mishap
git-aum looks for a config file called `.git-authors.toml` from the current directory up to root, so you could have a default `.git-authors.toml` file in your `$HOME` directory, or in your git repository
