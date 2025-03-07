# git-author-manager

tool to make managing author identities easier.

## usecases

this is useful to you, if you, for example:

- are peer programming on a shared machine, and want accurately attribute commits
  - `git aum doas <id> git commit (...)`
- have seperate author configurations for your personal and workplace account(s) and want to quickly configure a repo as either being a personal repo or a work related repo
  - with a `.git-authors.toml` in your `$HOME` directory, `git aum set <personal>` or `git aum set <some-company-name>`
- just want a simpler way to manage your `git config user.name` and `git config user.email`

## installation

installing with `cargo install`: `cargo install --git https://github.com/camper0008/git-author-manager`

## using

when installed, it ships a binary called `git-aum`, (git **au**thor **m**anager), which means you can run commands such as `git aum get` to get the current status

you can:
- `git aum get` to get the current config and authors available in the found config files
- `git aum set <id>` to set the git author to available author
- `git aum add <id> <name> email>` to add an author to the config
- `git aum remove <id>` to remove an author from the config
- `git aum doas <id> [cmd]...` to run a cmd as an author, and then change back to your previous config
- `git aum add-from-git <id>` to add an author based on your current git config
- `git aum copy-config [dest]` to copy the nearest config to `dest`, or the current working directory. if `dest` is a directory, it will write to `<dest>/.git-authors.toml`. if you have, for example, a `$HOME/.git-authors.toml`, you can use this to easily copy the config into your workspace.

there is a `-v, --verbose` flag for all of the commands which prints information such as which config file is being read from, aswell as a `git aum help` command, and a `-h, --help` flag.

### example

if you have a user called `tph`, and a user called `mtk` you can:
- set the author to tph: `git aum set tph`
- commit as mtk: `git aum doas mtk git commit -m "refactored board state"`
- and verify that you are still tph: `git aum get`

## poisoning global state

the software's git author management part works on a repository basis, and it only writes to the git config on the `local` level, i.e. a repository's `.git/config`, so it shouldn't corrupt your global config in the event of a mishap

## config files

git-aum looks for a config file called `.git-authors.toml` from the current directory up to `/`, so you could have a global `.git-authors.toml` file in your `$HOME` directory and still use it in your projects
