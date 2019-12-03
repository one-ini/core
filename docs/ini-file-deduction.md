# Deduction of the Core INI Format

Since the INI-file format isn't specified yet, the features of INI-files depend heavily on the used INI-parser and its configuration.

This is the attempt to create an overview of commonly used parsers and their interpretation of different INI-dialects to find a common feature-set which may be used as base for the strict specification of the EditorConfig-INI parser.

| Parser | Inline comments | Multiline Values | Escape Characters | Encoding Support | Empty Values | Trimmed Names | Example-Reference |
|---|---|---|---|---|---|---|---|
|[go-ini]| `X` | `X` | `X` | `X` | `-` | `?` | https://github.com/go-ini/ini/blob/master/testdata/full.ini |
|[inih]| `X` | `X` | `?` | `X` | `X` | `?` | https://github.com/benhoyt/inih/blob/master/tests/normal.ini |
|[iniparser]| `X` | `X` | `X` | `?` | `X` | `?` | https://github.com/ndevilla/iniparser/blob/master/test/ressources/good_ini/twisted.ini |
|[ini-parser]| `X` | `X` | `0` | `X` | `X` | `X` |  |
|[node-ini]| `X` | `X` | `0` | `X` | `X` | `X` |  |
|[bash-ini-parser]| `X` | `X` | `0` | `X` | `X` | `X` |  |
|[rust-ini]| `X` | `X` | `0` | `X` | `X` | `X` |  |
|[gcfg]| `X` | `X` | `0` | `X` | `X` | `X` |  |
|[]| `X` | `X` | `0` | `X` | `X` | `X` |  |



[go-ini]: https://github.com/go-ini/ini
[inih]: https://github.com/benhoyt/inih
[iniparser]: https://github.com/ndevilla/iniparser
[ini-parser]: https://github.com/rickyah/ini-parser
[node-ini]: https://github.com/npm/ini
[bash-ini-parser]: https://github.com/rudimeier/bash_ini_parser
[rust-ini]: https://github.com/zonyitoo/rust-ini
[gcfg]: https://github.com/go-gcfg/gcfg

## Additional Resources

- RFC 822?
