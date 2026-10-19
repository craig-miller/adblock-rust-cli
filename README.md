# adblock-rust-cli

Thin command-line wrapper around [`brave/adblock-rust`](https://github.com/brave/adblock-rust) that converts EasyList/ABP-format filter lists into Apple's WKContentRuleList JSON format — the format consumed by WebKit's `WebKitUserContentFilterStore` API.

Written to feed the WebKit-native content-filter pipeline used by [vimb-blocklist](https://github.com/craig-miller/vimb-blocklist), available through [gentoo-overlay](https://github.com/craig-miller/gentoo-overlay), but it's a general-purpose tool. Any WebKit-based application that consumes `WKContentRuleList` JSON can use its output.

## Usage

```
adblock-rust-cli /path/to/easylist.txt > easylist.json
```

Reads from stdin if no path is given:

```
curl -sSL https://easylist.to/easylist/easylist.txt \
    | adblock-rust-cli \
    > easylist.json
```

Progress statistics (rules emitted, rules that couldn't be represented in WebKit's format) go to stderr. The JSON array itself goes to stdout, always compact (WebKit does not require pretty-printing).

## Exit codes

- `0` — success
- `2` — failed to read input
- `3` — failed to serialize JSON

## Coverage

WebKit's content-blocker format is a subset of what EasyList can express. On EasyList itself, about 99% of rules convert cleanly; the ~1% that don't include rules with `$redirect=`, `$csp=`, `$removeparam=`, scriptlet injections (`+js(...)`), `:style(...)` cosmetic actions, entity-based hosts (e.g. `google.*`), and rules containing non-ASCII characters. Those get counted on stderr and dropped.

## License

MIT. Depends on `adblock` (MPL-2.0) and `serde_json` (MIT/Apache-2.0).
