# NPM

The [default](./binary.md#default-build) build of Taplo is published to NPM as [`@taplo/cli`](https://www.npmjs.com/package/@taplo/cli). You will need [Node.js](https://nodejs.org/en/) (tested with 16 and up) in order to use it.

This will install the `taplo` executable globally:

```sh
npm install -g @taplo/cli
```

```sh
yarn global add @taplo/cli
```

```sh
pnpm install -g @taplo/cli
```

## Run without installing

Alternatively you can run it once "without installing":

```sh
npx @taplo/cli --help
```

```sh
pnpm dlx @taplo/cli --help
```
