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
const { perform, PerformanceTime, FeedbackPattern } = require("@haptix/core");
// or
import { perform, PerformanceTime, FeedbackPattern } from "@haptix/core";

perform(FeedbackPattern.Generic, PerformanceTime.Now);
```

Note that nothing will happen unless you're touching the touchpad.

## Internal API? :D

There's an internal reverse-engineered API that's undocumented and may cause your app to be rejected from the Mac App Store.
It's WAY stronger vibration, and it will work without the user touching the trackpad.

```js
import { Actuation, internalApiPerform } from "./index.js";

internalApiPerform(Actuation.Strong);
```

### Internal-er API? :D

Sure ok here is an unsafe Rust function that calls an undocumented FFI API with unknown parameters.
This is likely to segfault :)

```js
import { internalApiPerform } from "./index.js";

// unsafeInternalApiPerform(actuation: integer number, unknown1: integer number, unknown2: floating point number, unknown3: floating point number): void
unsafeInternalApiPerform(6, 0, 0.0, 0.0);
```

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
