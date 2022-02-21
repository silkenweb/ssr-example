# Server Side Rendering Example

This example shows how to render web pages at compile time and hydrate them on first page load. The [xtask] package will build the app and render the initial pages. The [app] package contains the actual app. It's split into a library and `main` so the code can be shared with [xtask] and [app]. [xtask] is based on the [xtask concept].

## Run

```bash
cargo xtask serve
```

Then point your browser at <http://127.0.0.1:8000/>.

[xtask]: packages/xtask
[app]: packages/app
[xtask concept]: https://github.com/matklad/cargo-xtask/
