# haptix

**haptix:** MacOS touchpad haptics

## Installing haptix

Installing haptix currently requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm/pnpm/yarn.

```sh
$ npm install haptix
$ pnpm install haptix
$ yarn install haptix
```

## How do I use it?

There's only one function!

```js
const { perform, PerformanceTime, FeedbackPattern } = require("haptix");
perform(FeedbackPattern.Generic, PerformanceTime.Now);
```

Note that nothing will happen unless you're touching the touchpad.

## Building haptix from source locally

After cloning this repo, simply run

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

### Available Scripts

In the project directory, you can run:

#### `npm install`

Installs the project, including running `npm run build`.

#### `npm build`

Builds the Node addon (`index.node`) from source.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm build` and `npm build-*` commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm build-debug`

Alias for `npm build`.

#### `npm build-release`

Same as [`npm build`](#npm-build) but, builds the module with the [`release`](https://doc.rust-lang.org/cargo/reference/profiles.html#release) profile. Release builds will compile slower, but run faster.

#### `npm npm-test`

Runs a JS demo of the library.

# Other Links

This project was built using Node, Rust, and Neon.

To learn more about Neon, see the [Neon documentation](https://neon-bindings.com).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).
