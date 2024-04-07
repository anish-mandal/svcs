# svcs

> 🚧 IN DEVELOPMENT 🚧

svcs is a very simplified and blazingly fast version control system (inspired by git) focusing on only the essential features of a version control system.

## Features

- [x] init
- [ ] commit
- [ ] checkout
- [ ] branch
- [ ] tag
- [ ] subcommit
      _more coming soon..._

## File Structure

Following shows the file structure svcs will initialize the `.svcs` directory:

```fs
.svcs
`- master
	`- blobs
	`- commits
	`- structures
```

`master` is the default branch name. `blobs` stores the compressed files from the project. `structues` stores the compressed (deflated) [ron](https://github.com/ron-rs/ron) files which describes the file structure for a particular commit.
