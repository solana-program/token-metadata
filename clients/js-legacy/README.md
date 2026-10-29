# `@solana/spl-token-metadata`

A TypeScript interface describing the instructions required for a program to implement to be considered a "token-metadata" program for SPL token mints. The interface can be implemented by any program.

## Links

- [TypeScript Docs](https://solana-labs.github.io/solana-program-library/token-metadata/js/)
- [FAQs (Frequently Asked Questions)](#faqs)
- [Install](#install)
- [Build from Source](#build-from-source)

## FAQs

### How can I get support?

Please ask questions in the Solana Stack Exchange: https://solana.stackexchange.com/

If you've found a bug or you'd like to request a feature, please
[open an issue](https://github.com/solana-program/token-metadata/issues/new).

## Install

```shell
npm install --save @solana/spl-token-metadata @solana/web3.js@1
```
_OR_
```shell
yarn add @solana/spl-token-metadata @solana/web3.js@1
```

## Build from Source

0. Prerequisites

* Node 16+
* PNPM 9+

1. Clone the project:
```shell
git clone https://github.com/solana-program/token-metadata.git
```

2. Navigate to the library:
```shell
cd token-metadata/clients/js-legacy
```

3. Install the dependencies:
```shell
pnpm install
```

4. Build the library:
```shell
pnpm build
```

5. Run the tests:
```shell
pnpm test
```
