# haptix

**haptix:** MacOS touchpad haptics

## Installing haptix

`haptix` is only supported on MacOS.

You can install the project with npm/pnpm/yarn.

```sh
$ npm install haptix
$ pnpm install haptix
$ yarn install haptix
```

## How do I use it?

There's only one method!

```js
const { perform, PerformanceTime, FeedbackPattern } = require("haptix");
// or
import { perform, PerformanceTime, FeedbackPattern } from "haptix";

perform(FeedbackPattern.Generic, PerformanceTime.Now);
```

Note that nothing will happen unless you're touching the touchpad.

## Building haptix from source locally

After cloning this repo, simply run

```sh
$ yarn build
```

# Other Links

This project was built using Node, Rust, and napi-rs.

To learn more about napi, see the [napi documentation](https://napi.rs/docs/introduction/simple-package).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).
