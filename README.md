# keep [<img src="https://travis-ci.org/nathansizemore/keep.svg?branch=master">][travis-badge]

Simple lightweight storage and retrieval program.

---

## Usage

```
$ keep save --tag=todo "Fix all the bugs."

$ keep list
| id | tag  | item              |
---------------------------------
| 1  | todo | Fix all the bugs. |

$ keep --help
keep - it keeps shit, so you can look at it later.

Usage:
    keep save [--tag=<t>] <item>
    keep list [--tag=<t>]
    keep rm [--all] [--tag=<t>] [<id>]
    keep -h | --help
    keep --version

Options:
    -a --all        Apply command to every entry.
    -t --tag=<t>    Name to help identify item.
    -h --help       Show this screen.
    --version       Show version.
```

---

### Author

Nathan Sizemore, nathanrsizemore@gmail.com

### License

keep is available under the MPL-2.0 license. See the LICENSE file for more info.



[travis-badge]: https://travis-ci.org/nathansizemore/keep
