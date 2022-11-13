# gid

> Git ID Manager

`gid` is a Git identity manager that helps switch between multiple "profiles"
of Git configurations. `gid` is non-intrusive, injecting configuration options
on a per-command basis rather than editing local or global Git configurations.
Furthermore, `gid` is a cross-platform utility that only requires an available
`git` command in the environment as a dependency.

## Usage

### Configuration

Before `gid` or its accompanying utility `gidc` are used, a configuration file
must be created.

This file must be named `gid.toml`, and can be placed in one of two locations
(listed in order of priority):

1. Any path pointed to by an environment variable `GID_CONFIG`
2. In the current working directory where the `gid` or `gidc` commands are used

An example `gid.toml`:

```toml
active = "profile_name_1"

[profile.profile_name_1]
user.name = "my_git_username"
user.email = "my_git_email@example.com"
user.signingkey = "my_git_signingkey"
commit.gpgsign = true
tag.gpgsign = true
pull.rebase = false
sshkey = "$HOME/.ssh/my_ssh_key"

[profile.whatever_other_name]
user.name = "my_git_username_2"
user.email = "i-only-want-to-change-these-settings@example.com"
sshkey = "$HOME/.ssh/id_rsa"
```
