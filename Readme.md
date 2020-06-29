# up

## WIP - unusable at the moment


# Todo
Bold are important

- *add customizable cloudflare/google dns resolvers*
- generate dhparams and use them
- *logging*
- decide fallback index.html or 404
- print help message when no sub command is provided
- location should be configurable instead of just /
- add too many tests
- client max body size configurable?
- *Multiple sites at once*
- *generate configuration only, at current directly* (user may not want to just override the /etc/nginx directly without seeing what tool is doing first)
- *make UI interesting please*


# Notes
- root is not required when reverse proxy is enabled
- #[cfg(unix)] for main function? won't be supporting windows anyhow
- Assumptions:
  - `/etc/nginx` is going to be default path, and other folder structure of nginx is unchanged
  - `tar`, `openssl` and `certbot` binaries are in $PATH

# next steps
