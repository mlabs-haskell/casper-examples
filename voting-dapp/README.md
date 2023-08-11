# Voting dApp

- [Voting dApp](#voting-dapp)
  - [Project description](#project-description)
  - [Repo structure](#repo-structure)
    - [3d-party-contract](#3d-party-contract)
    - [nctl-docker](#nctl-docker)
    - [node-proxy](#node-proxy)
    - [testnet-keys](#testnet-keys)
    - [voting](#voting)
    - [voting-frontend](#voting-frontend)
  - [Contract on-chain and backend](#contract-on-chain-and-backend)
    - [Odra framework](#odra-framework)
      - [Odra pros](#odra-pros)
      - [Odra cons](#odra-cons)
    - [Codebase](#codebase)
  - [Contract frontend](#contract-frontend)
    - [Signing](#signing)
  - [Deploying the project](#deploying-the-project)
    - ['Resetting' contract state](#resetting-contract-state)
    - [Switching the network](#switching-the-network)
    - [Testnet deploy](#testnet-deploy)
      - [Step 1 - keys](#step-1---keys)
      - [Step 2 - build and test](#step-2---build-and-test)
      - [Step 3 - prepare environment](#step-3---prepare-environment)
      - [Step 4 - deploy governor](#step-4---deploy-governor)
      - [Step 5 - query service](#step-5---query-service)
      - [Step 6 - node proxy](#step-6---node-proxy)
      - [Step 7 - frontend](#step-7---frontend)
      - [Step 8 - 3d-party-contract](#step-8---3d-party-contract)
      - [Step 9 - interact with the contract](#step-9---interact-with-the-contract)

## Project description

This is an example of a full-stack project that implements a simple DAO contract. The on-chain part is written with the [Odra framework](https://odra.dev/docs/), which greatly simplifies contract writing but also has some drawbacks (see the [Contract on-chain and backend section](#contract-on-chain-and-backend)). User interaction occurs through a React application.

After the main contract is deployed on-chain, users can:

- Create new proposals. Currently, a proposal consists of a description and a call to some arbitrary 3rd-party contract. From on-chain and backend parts there are no limits on what contract entry point with what arguments can be called. However, the frontend UI currently has some limitations on what arguments can be passed to the contract entry point. For more details, see the [Contract frontend section](#contract-frontend).
- Vote on created proposals. Currently, there are no access restrictions and anybody can vote, but only once per proposal. The initial idea was to allow users to vote according to their stake represented by some ERC20 standard token, but this was omitted due to development time limitations.
- Close voting. There are no access limits currently - anybody can close voting. If a proposal receives a majority of "YES" votes, the contract call inside the proposal will be executed, and the person who closes the voting will need to pay the gas price to cover whatever was executed there.

## Repo structure

### 3d-party-contract

Here, you will find a simple smart contract that will be used for demo purposes. The contract is written in vanilla/default/low-level Casper. The directory also contains shell scripts to deploy this contract and query the node using `casper-client` (analogous to `cardano-cli`).

### nctl-docker

This directory contains a Docker Compose setup to start a local private network using the `nctl` tool provided by the Casper ecosystem. There are `Make` commands available to start, stop, and restart the network, as well as to copy predefined funded keys. The keys are already copied into the [nctl-docker/users](./nctl-docker/users/) directory, but if the node version changes, the old keys may stop working. The node version can be set through the [docker-compose file](./nctl-docker/docker-compose.yaml).

### node-proxy

This TypeScript proxy server solves the issue of proxying requests from a browser frontend for Casper nodes that require CORS. For more details, please refer to the [Deploying the project section](#deploying-the-project).

### testnet-keys

This directory contains funded keys on the `testnet` network. These keys can be imported into the Casper Wallet browser extension.

### voting

This directory contains two Rust packages:

- A DAO smart contract with a deployer implemented using the Odra framework.
- A query service that enables you to query the network's global state in relation to the contract's Context.

For more details see [Contract on-chain and backend section](#contract-on-chain-and-backend)

### voting-frontend

Here, you will find a React app with a basic user interface that allows you to create new proposals, vote on them, and close them. See [Contract frontend section](#contract-on-chain-and-backend)for details.

## Contract on-chain and backend

The Voting Smart contract is implemented using the [Odra framework](https://odra.dev/docs/), which abstracts away all low-level Casper code. Additionally, Odra generates a `Deployer` that provides a reference object after the contract is deployed. This reference object can be used to call contract entry points and query the global state of the contract in tests and using the real network.

### Odra framework
For a more comprehensive example of using the Odra framework in a core ecosystem project see: [Casper DAO contracts](https://github.com/make-software/dao-contracts).

#### Odra pros

- A lot of low-level code is abstracted away. In bigger projects, you will probably want to abstract out low-level Casper code anyway to avoid writing a lot of boilerplate. So Odra gives you that already.
- Code looks more like plain Rust: the contract is a `struct` and contract endpoints are public methods of `impl`. Also, all storage interactions are hidden behind `struct` fields that mimic regular types like variables of type `T`, lists, maps, and so on.
- Tests are kept in the same `.rs`, not in a separate package like in "vanilla" Casper examples. It is also possible to run tests either with Odra mock VM or with the "official" Casper mock VM. Odra tests give slightly better error messages, but for a final check, I would go with Casper VM tests followed by E2E test on a local private network.
- `Deployer` is generated for each contract, providing a simple abstraction for calling contract entry points. They are called just like regular methods via dot-notation. The `livenet` feature allows deploying and calling contracts on a real network.
- Has built-in support for events with some quality of life improvements (uses [casper-event-standard](https://github.com/make-software/casper-event-standard)).

#### Odra cons

Odra cons comes from the way it abstracts low-level Casper code. The framework generates names for `NamedKeys` and `Dictionary` keys used to store data on-chain. In low-level Casper code contracts, developers define a set of string constants for `NamedKeys` names and `Dictionaries` to use for storing contract state. Odra, on the other hand, stores the entire contract state in a single `Dictionary` (currently called `state`). Key names for this `Dictionary` are generated by Odra and this process is hidden from the developer.

For version `v0.4.0`, if a contract `struct` has a `Variable`, the value is stored inside the `state` dictionary, not as a separate `Named Key`. The key for the dictionary is generated by concatenating the contract name with the variable name, converting the resulting string to bytes, and hashing it. The hex-encoded hash is then used as a key in the `state` `Dictionary`. If one contract has another contract as its field, both contract names are concatenated, and the variable name is added to them. The resulting string is converted to bytes and hashed. If something is stored in the `Mapping`, the algorithm is even more involved. The sources for how keys are created for variables and dictionary types like `Mapping` can be found [here](https://github.com/odradev/odra/blob/release/0.4.0/odra-casper/shared/src/key_maker.rs#L12) and [here](https://github.com/odradev/odra/blob/release/0.4.0/odra-casper/livenet/src/casper_client.rs#L397), respectively.

The fact that the algorithm for generating keys is hidden from the contract developer and subject to change causes at least two problems:

- Key names of the data stored on-chain are not transparent. When writing low-level Casper code, the developer defines the names explicitly, and these names can be inspected in the smart contract source code. If we query the contract state, we will see the same keys and dictionary names in the contract context. However, with Odra, we will only see a single dictionary called `state`.
- Querying data from the smart contract becomes more difficult. To use `casper-js-sdk` or `casper-client`, we need to know the exact names for `Named Keys` and the `Dictionary`, as well as the string key for the `Dictionary` item to query. However, Odra does not expose generated key names to the developer in any way and does not allow developers to set their own names for variables.

It is not clear how to fix the first issue, unless Odra allows custom keys to be set for contract variables.

For the second issue, several approaches can work:

- When using `casper-js-sdk` on the frontend, it is possible to replicate the way ODra generates keys in JS/TS code. This approach was tested for `v0.4.0` and worked, but the algorithm for key names generation can change, so this approach may require additional maintenance.
- Add getters to your Odra contract for each field you want to query. Then, reference to the contract provided by autogenerated `Deployer` will also have this getters, and they can be called via dot-notation like contract entry points. Although under the hood, these getters will be interpreted as JSON RPC calls to the node that do not require any gas. The question now is how to make those getters available for the frontend. With the current Odra version `0.4.0`, there is no out-of-the-box solution. As a workaround, for this project a simple [web service (query-service)](./voting/query-service/) has been written to provide a REST API on top of the contract getters.

Other possible ways to fix the query issue:

- In release `0.6.0`, the Odra team plans to add a [WASM client](https://github.com/odradev/odra/issues/202) which will be auto-generated from the contract getters (or maybe straight from `struct` fields) and can be run in the browser.
- Emit events when the contract state changes. Those events can be indexed by some custom indexer, and then the frontend can query this indexer. It seems like [casper.cloud](https://cspr.cloud/) can become a general solution for it (it is probably one of the results of the [casper-dao-middleware](https://github.com/make-software/casper-dao-middleware) development).

### Codebase

The Contract on-chain and backend have their own Makefile in the [root directory](./voting/). The commands provided there allow for building contracts, testing them with both Odra-mock and Casper VMs, building and running the query service, deploying contracts via Odra's `livenet` feature, and running E2E tests. For more details on deployment, please refer to the [Deploying the project section](#deploying-the-project).

Currently, the contract does not use any Events.

## Contract frontend

Frontend is React application written in TypeScript with very basic UI. This is my first experience both with React and TypeScrip, so I suspect some things there are "pretty suboptimal" 🙃.

Application uses [casper-js-sdk](https://github.com/casper-ecosystem/casper-js-sdk) to build ans submit deploys. `casper-js-sdk` related code mostly concentrated in the [GovernorClient.ts](./voting-frontend/src/GovernorClient.ts).

### Signing

It is possible to sign deploys either with Casper Wallet browser extension or with some known keys, see [CasperNetwork.ts](./voting-frontend/src/CasperNetwork.ts). To switch the way of signing use `USE_CASPER_WALLET` constant in [Settings.ts](./voting-frontend/src/Settings.ts). There are some hardcoded keys in [Utils.tsx](./voting-frontend/src/Utils.tsx) that are parsed from Base64 encoded secret key. Those keys were used for development and debugging. Adjust the module if needed. It is also possible to parse keys from `.pem` files using `casper-js-sdk`.

## Deploying the project

Thw following section describes the procedure for deploying the `Governor` contract on the testnet, creating a proposal that will call a 3rd-party contract, and voting on it.

Please read additional instructions in [Switching the network section](#switching-the-network) if you want to try to run project with the local private network, as Casper Wallet browser extension can not connect to the custom network and you will need Base64 encoded secret key or secret key `.pem` file to sign deploys. 

### 'Resetting' contract state

If you want to "reset" state of the contracts described below, just re-deploy them with the same account key. With [nctl-docker local network](./nctl-docker/) there is also [make command available](./Makefile) to reset the network.

### Switching the network

To switch current setup from the testnet several changes are required:

- Change data [.env file in contract dir](./voting/contracts/.env) to use correct network name, node address and proper key. E.g., [data for NCTL local network](./voting/contracts/.env.ln). `query-service` will copy and use this file if started via `make` command.
- Change [frontend settings](./voting-frontend/src/Settings.ts) accordingly. Important: for `NODE_URL` change only the part after the proxy url, e.g. for NTCL docker setup it will be `NODE_URL = 'http://localhost:3001/http://localhost:11101'`, and if you are using local network, then switch `USE_CASPER_WALLET` to `false`, as Casper Wallet extension can not connect to the custom network - deployments will be auto-signed with the key from [Utils.theKeys()](./voting-frontend/src/Utils.tsx#9)

### Testnet deploy

(Step 0: prepare a lot of terminals)

#### Step 1 - keys

To deploy on testnet you will need keys. Keys can be created from Casper Waller browser extension, or using [casper-client](https://docs.casper.network/concepts/accounts-and-keys/). Keys created with Casper client can be imported to Casper Wallet browser extension. 

One secret key `.pem` file will be required to deploy the main contract with the current setup. It is possible to download keys from Casper Wallet browser extension if needed.

After keys are created and added to Casper Wallet you can use [faucet](https://testnet.cspr.live/tools/faucet) to request funds (only one request per address is allowed).

There are some funded keys awaitable in the repo in [testnet-keys dir](./testnet-keys/).

 It is required step as Casper Wallet will be used to sign deployments.

#### Step 2 - build and test

Build and test contracts.
```
cd voting
make test-w-casper
```

There should be no errors.

#### Step 3 - prepare environment

Setup Odra `livenet` environment.

Odra `livenet` feature allows you to deploy and call contracts on the real network right from еру Rust project. Path to the secret key, node url and network name are specified through the [.env file](./voting/contracts/.env). The file is currently set for testnet. `http://94.130.10.55:7777` is the RPC endpoint for some known public node running on testnet.

The is also example for `nctl` docker local network setup in `.env.ln`

#### Step 4 - deploy governor

Deploy main contract - the governor, using Odra `livenet`.

You should be in the `voting` directory.

```
make deploy-via-livenet
```

If everything went OK you should see output like this:

```
💁  INFO : Deploying "governor.wasm".
💁  INFO : Found wasm under "wasm/governor.wasm".
🙄  WAIT : Waiting 15s for "ccd895b021dcb1122032e3a87840c06bd6371a1eb32463b73fd68a689eabb5b3".
🙄  WAIT : Waiting 15s for "ccd895b021dcb1122032e3a87840c06bd6371a1eb32463b73fd68a689eabb5b3".
🙄  WAIT : Waiting 15s for "ccd895b021dcb1122032e3a87840c06bd6371a1eb32463b73fd68a689eabb5b3".
🙄  WAIT : Waiting 15s for "ccd895b021dcb1122032e3a87840c06bd6371a1eb32463b73fd68a689eabb5b3".
💁  INFO : Deploy "ccd895b021dcb1122032e3a87840c06bd6371a1eb32463b73fd68a689eabb5b3" successfully executed.
💁  INFO : Contract "hash-d5e09f8a3836faf50dd0fc416784818ab17da481d6e3f3b2e01539270432b0cc" deployed.
```

You can check deployment hash and contract hash via https://testnet.cspr.live (deploy hash is printed to the terminal in messages like `WAIT : Waiting 15s for "..."`).

During contract deployment `governor.json` file will be created in the `voting` directory. This file will contain contract package hash (but not contract hash - this is important and will be explained down the road). This file will be used by `query-service` so frontend can query it and figure out what contract to call. It was made mostly to speed up develop-debug loop.

#### Step 5 - query service

Start query service.

You should be in the `voting` directory.

```
make run-query-service
```

`query-service` uses "getters" provided by the governor contract to query node, so environment setup for `livenet` Odra feature is also required here. When `make run-query-service` is executed, it copies `.env` file from the `contracts` directory into own `query-service` directory to keep them in sync. Then it starts web-service on port `8080` built with `Actix` Rust library.

#### Step 6 - node proxy

Casper nodes require CORS. It was told by developers, that starting from version `1.5` cors will be removed, and it was indeed till `1.5.2`. At the current moment testnet nodes run `1.5.2`, so CORS is back.

The easiest way to deal with CORS I;ve found at least for development, is to use small TS server with `cors-anywhere`.

You can see in the frontend repo in [Settings.ts](./voting-frontend/src/Settings.ts) `NODE_URL` is specified as an url for proxy server with node url as an argument. This allows to use node  and contract clients provided by `casper-js-sdk` as is at the fronted w/o writing whole bunch of own code.

Perhaps, this proxy can be merged with `query-service`, but if Odra team will release `WASM` contract client mentioned in `Other possible variants` of [Odra cons section](#odra-cons), there will be no need in `query-service`. So I went with standalone proxy for now.

Leave it running.

#### Step 7 - frontend

Switch to [voting-frontend dir](./voting-frontend/).

Run

```
npm start
```

or to prevent default browser launch

```
BROWSER="none" npm start 
```

Frontend app will start at `http://localhost:3000/`. You should see couple text fields and form with button there.

Frontend requires governor contract deployed, `proxy` and `query-service` running. If you want to test contract execution, then `3d-party-contract` should be deployed also. If you followed all steps in order, everything should be prepared already.

#### Step 8 - 3d-party-contract

3d-party contract can be used to test arbitrary contract execution as the result of voting. It is written in low-level Casper and mull code included for reference in [main.rs file](./3d-party-contract/main.rs).

The contract stores single variable on-chain that can be incremented by `counter_inc` entry point by desired amount. Amount is passed through entry point arguments.

All interactions happens thorough shell scripts. Scripts use `casper-client` CLI tool, so you will need it installed.

Enter [3d-party-contract contract directory](./3d-party-contract/) in terminal.

Pick user key to work with and initiate environment. E.g. running `source ./test-net-debug-user-env` will export environment variable for testnet node and secret key that have some casper funds - it was used for debugging.

Call deployment with

```
./deploy-contract.sh
```

You should see something like this:

```
{
  "id": -3930619943095932514,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.5.2",
    "deploy_hash": "78bc3f41b7d0a51f291cab8c2e4260a679959c5f37c547cfca4dc7c90b5e3c98"
  }
}
```

Now you can check deploy hash either via `testnet.cspr.live` or by calling `./get-deploy.sh "THE_HASH"`. `testnet.cspr.live` shows you the status in UI. In case of  `./get-deploy.sh ...` seek for `execution_results` key. First `result` in array will contain either `Success` or `Failure`. `Failure` usually contains the reason.

Now we can query account to figure out Contract package hash - we will need it:

```
./query-whole-state.sh
```

Look `named_keys` for something like this:
```
{
  "key": "hash-9556e2bc1477dfce434f1b1768f496792d8059b4746c2815bd52ac7ae6cad66b",
  "name": "counter_package_hash"
}
```

This is the hash we will need later.

(Leave this terminal open - we will need it 🙂)

#### Step 9 - interact with the contract

After frontend is launched, 1st thing you should to is to click `Init` button. Then following happens:

- App will request access to Casper account and set required data to the state. Casper Wallet will open asking to choose and connect account. I had no time to implement account switching according to the events from Casper Waller SDK, so if account is switched - press `Init` again. Also, when page is refreshed you may need to click `Init` to set key again - this is my bad - lacking of experience and couldn't figure out how to make it better in the time I had. Also, sometimes I see error `CasperWalletProvider is not a function` - don't know the cause, page reload w/o cache helps.
- After key is set application will make request to `query-service` to get `package hash` of the governor contract. Then it will use `CasperClient` from `casper-js-sdk` to find `contract hash` by `package hash`. For whatever reason `Contract` client from `casper-js-sdk` needs `contract hash` to call the contract, and fails with `package hash`. Casper natively support contracts upgrade, and different versions of contracts wit different hashes are "stored under" `package hash`. So user can call specific version of contract using `package hash` and version, or just call latest version with just `package hash`. Odra uses `package hash` to call contracts, `casper-client` can use both. But `casper-js-sdk` needs `contract hash`.

If everything went OK, you should see current public key hash, contract hash and package hash above of the form.

Now to the proposals. Using the below you can create proposals.

From left to right you can specify (there is no validation there really):

- Proposal description
- Information required to call external contract. This information will be stored on-chain as part of the proposal. Call will be executed if proposal receives majority of "YES" votes:
  - Package hash of the contract. Remember, that Odra uses `package hash` to call contracts. So use hash acquired during [3d-party-contract-deploy](#step-8---3d-party-contract).
  - Contract entry point to call. In case of `3d-party-contract` it will be `counter_inc`
  - Argument for the entry point. `counter_inc` accepts number. I had no time to make it possible to pass any set of arguments supported by contract entry points though the form, so here arguments type is limited by `number`. There is no limits on smart contract side - it will accept any serialized arguments, that frontend will supply to it. But currently frontend code has only arguments required for `counter_inc` hardcoded - see [submit function in NewProposal.tsx](./voting-frontend/src/NewProposal.tsx).

After form is filled, click `Add Proposal`. It will ask you to sign deploy with Casper Wallet extension and then send deploy to the node. You can monitor the process in browser console, but alert will also popup when deployment will succeed or fail.

When deploy is finished, you can refresh the page so app will query proposals. They should appear under the form. From here you can one on them (one vote per key) nad then finalize voting. If proposal received majority of "YES" votes, contract call stored inside the proposal will be execute. If it fails you will see an error and proposal will stay un-finalized (with current logic).

If call to the `3d-party-contract` was used (and you don't have that many options with the current frontend 🙂) and it was executed during finalization, you can check if call to `3d-party-contract` was executed successfully.

Go back to `3d-party-contract` directory and from there run:
  
```
./query-state.sh "counter/count"
```

You should see something like this:

```
  account-hash-b18e832a195ae7c01984f9830db5bf195e615bb335489364022a1d6525545832
66ca03c819a00b940128682a7d2d406ef1953e3a9adbf10662af513ff7bb8cda
{
  "id": -6000593732474252967,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.5.2",
    "block_header": null,
    "merkle_proof": "[94662 hex chars]",
    "stored_value": {
      "CLValue": {
        "bytes": "00000000",
        "cl_type": "I32",
        "parsed": 0
      }
    }
  }
}
```

The `"parsed": X` is what should change by the amount you specified in the proposal form.

Now you can create more proposal, vote on them and finalize them. Just don't forget to refresh the page, as it updates only fresh query response from `query-service` (and probably you'll need to click `Init` again after that - sorry).
