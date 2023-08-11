# Casper project template

- [Casper project template](#casper-project-template)
  - [Overview](#overview)
  - [Smart contract](#smart-contract)
  - [TypeScript client](#typescript-client)


## Overview
This is an opinionated improved version of the default template available through `cargo casper my-project` (see [getting started docs](https://docs.casper.network/developers/writing-onchain-code/getting-started/)). The main improvement is that the [contract](./contract/) package is set up to be used as a library in the [tests](./tests/) package. This allows constants containing entry point names or `Named Keys` names, helper functions, and any other reusable code to be imported from the smart contracts package to the tests package. In the default Casper template, values are copy-pasted from contracts to tests, which is cumbersome to maintain and doesn't allow for sharing functions or other code from the contracts package.

The current codebase structure differs from the default Casper template and can serve as a more convenient starting point for new projects. However, it is not set in stone and should be adopted based on the size and complexity of the project. For large and complex objects, you may want to abstract some Casper low-level boilerplate code into reusable abstractions. A significant portion of this work has already been done by the [Odra framework](https://odra.dev/docs), which is actively being developed. Although it comes with some caveats, it is already being used in some cornerstone Casper ecosystem projects (such as [dao-contracts](https://github.com/make-software/dao-contracts)`). For examples of MLabs' internal experience using Odra, see the examples provided - #TODO

In addition to the smart contracts, the codebase also includes a TypeScript client for deploying and calling smart contracts to build the frontend. It also provides a local private network setup via Docker.

## Smart contract

The following example showcases a simple smart contract. Upon deployment, users can call the entry point to "register" their own public key `Address`. Once the `Address` is registered, users can append arbitrary strings to the string stored on-chain via another entry point. If a user's public key is not registered, the contract will fail. Note that "registering" is implemented by adding the key to the on-chain `Dictionary`, and serves only as an example of how to work with dictionaries. Casper smart contracts have native mechanisms to restrict access to contract entry points. Additionally, there is a separate entry point that can be used to emit an `event` with a user-defined message.

The on-chain example code demonstrates the following patterns and capabilities of smart contracts:

- The `init` function pattern: session code inside the main entry point `pub extern "C" fn call(){...}` calls a developer-defined contract initialization function immediately after the contract is added to the global state. `init` is the regular entry point of the contract used to set the initial state of the Contract inside the `Contract` context. It can have any name, but developers should implement some mechanism to prevent undesired subsequent calls of this function after the contract is deployed.
- Code on how to store and retrieve data with `Named Keys`.
- Code on how to store and retrieve data with `Dictionary`.
- The usage of events via [casper-event-standard](https://github.com/make-software/casper-event-standard). The Casper network does not support events natively, so they are implemented as a standalone library (using `Dictionary` under the hood).
- The [tests package](./tests/) contains code snippets for working with contract calls, global storage, and events in the "vanilla" Casper test suite.

## TypeScript client

The on-chain code is accompanied by a TypeScript client which utilizes the `casper-js-sdk`. The main entry point is `TestScenario.ts`, which runs an end-to-end test that deploys a contract, subscribes to events, registers an `Address`, appends a string to the value stored on-chain, and emits a couple of events. The events are then logged to the console.

It is common in Casper repositories to hide smart contract deployment and entry point calls behind an "SDK client". In this case, `ExampleContractClient.ts` was created for this purpose.

The `@make-software/ces-js-parser` package is used to parse events.
