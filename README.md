# pause-rs

This is an experiment to create a tiny pause container following
[these instructions](https://github.com/johnthagen/min-sized-rust/blob/fdfd162f63a1e6dbaad4e866dc0d3089f4214a1e/README.md).

// TODO(jdt): add further details about test setup, etc

<table>
<tr>
<td>comparison</td><td>size (Kb)</td>
<tr>

<tr>
<td>registry.k8s.io/pause:3.9</td><td>744</td>
<tr>

<tr>
<td>`cargo build --bin original`</td><td>6612</td>
<tr>

<tr>
<td>`cargo build --bin original --profile release`</td><td>4176</td>
<tr>

<tr>
<td>`cargo build --bin original --profile strip`</td><td>347</td>
<tr>

<tr>
<td>`cargo build --bin original --profile optz`</td><td>355</td>
<tr>

<tr>
<td>`cargo build --bin original --profile lto`</td><td>299</td>
<tr>

<tr>
<td>`cargo build --bin original --profile cgu`</td><td>299</td>
<tr>

<tr>
<td>`cargo build --bin original --profile panic`</td><td>295</td>
<tr>

<tr>
<td>`cargo build --bin nomain --profile panic`</td><td>279</td>
<tr>

<tr>
<td>

```
cargo +nightly build \
    -Z build-std=std,panic_abort \
    --target x86_64-unknown-linux-gnu \
    --profile panic \
    --bin nomain
```

</td><td>211</td>
<tr>

<tr>
<td>

```
cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --profile panic \
    --bin nomain
```

</td><td>26</td>
<tr>

<tr>
<td>

```
cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu \
    --profile panic \
    --bin nocorefmt
```

</td><td>26</td>
<tr>

<tr>
<td></td><td></td>
<tr>
</table>
