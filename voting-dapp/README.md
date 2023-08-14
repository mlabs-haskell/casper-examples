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

Here, you will find a React app with a basic user interface that allows you to create new proposals, vote on them, and close them. See [Contract frontend section](#contract-on-chain-and-backend) for details.

## Contract on-chain and backend

The Voting Smart contract is implemented using the [Odra framework](https://odra.dev/docs/), which abstracts away all low-level Casper code. Additionally, Odra generates a `Deployer` that provides a reference object after the contract is deployed. This reference object can be used to call contract entry points and query the global state of the contract in tests and while using the real network.

### Odra framework
For a more comprehensive example of using the Odra framework in a core ecosystem project see: [Casper DAO contracts](https://github.com/make-software/dao-contracts).

#### Odra pros

- A lot of low-level code is abstracted away. In bigger projects, you will probably want to abstract out low-level Casper code anyway to avoid writing a lot of boilerplate. So Odra gives you that already.
- Code looks more like plain Rust: the contract is a `struct` and contract endpoints are public methods of `impl`. Also, all storage interactions are hidden behind `struct` fields that mimic regular types like variables of type `T`, lists, maps, and so on.
- Tests are kept in the same module as contracts, not in a separate package like in "vanilla" Casper examples. It is also possible to run tests either with Odra mock VM or with the "official" Casper mock VM. Odra tests give slightly better error messages, but for a final check, I would go with Casper VM tests followed by E2E test on a local private network.
- `Deployer` is generated for each contract, providing a simple abstraction for calling contract entry points. They are called just like regular methods via dot-notation. The `livenet` feature allows deploying and calling contracts on a real network.
- Has built-in support for events with some quality of life improvements (uses [casper-event-standard](https://github.com/make-software/casper-event-standard)).

#### Odra cons

Odra cons comes from the way it abstracts low-level Casper code. The framework autogenerates names for `NamedKeys` and `Dictionary` keys used to store data on-chain. In low-level Casper code contracts, developers define a set of string constants for `NamedKeys` names and `Dictionaries` to use for storing contract state. Odra, on the other hand, stores the entire contract state in a single `Dictionary` (currently called `state`). Key names for this `Dictionary` are generated by Odra and this process is hidden from the developer.

For version `v0.4.0`, if a contract `struct` has a `Variable`, the value is stored inside the `state` dictionary, not as a separate `Named Key`. The key for the dictionary is generated by concatenating the contract name with the variable name, converting the resulting string to bytes, and hashing it. The hex-encoded hash is then used as a key in the `state` `Dictionary`. If one contract has another contract as its field, both contract names are concatenated, and the variable name is added to them. The resulting string is converted to bytes and hashed. If something is stored in the `Mapping`, the algorithm is even more involved. The sources for how keys are created for variables and dictionary types like `Mapping` can be found [here](https://github.com/odradev/odra/blob/release/0.4.0/odra-casper/shared/src/key_maker.rs#L12) and [here](https://github.com/odradev/odra/blob/release/0.4.0/odra-casper/livenet/src/casper_client.rs#L397), respectively.

The fact that the algorithm for generating keys is hidden from the contract developer and subject to change causes at least two problems:

- Key names of the data stored on-chain are not transparent. When writing low-level Casper code, the developer defines the names explicitly, and these names can be inspected in the smart contract source code. If we query the contract state, we will see the same keys and dictionary names in the contract context. However, with Odra, we will only see a single dictionary called `state`.
- Querying data from the smart contract becomes more difficult. To use `casper-js-sdk` or `casper-client`, we need to know the exact names for `Named Keys` and the `Dictionary`, as well as the string key for the `Dictionary` item to query. However, Odra does not expose generated key names to the developer in any way and does not allow developers to set their own names for variables.

It is not clear how to fix the first issue, unless Odra allows custom keys to be set for contract variables.

For the second issue, several approaches can work:

- When using `casper-js-sdk` on the frontend, it is possible to replicate the way Odra generates keys in JS/TS code. This approach was tested for `v0.4.0` and worked, but the algorithm for key names generation can change, so this approach may require additional maintenance.
- Add getters to your Odra contract for each field you want to query. Then, reference to the contract provided by autogenerated `Deployer` will also have this getters, and they can be called via dot-notation like contract entry points. Although under the hood, these getters will be interpreted as JSON RPC calls to the node that do not require any gas. The question now is how to make those getters available for the frontend. With the current Odra version `0.4.0`, there is no out-of-the-box solution. As a workaround, for this project a simple [web service (query-service)](./voting/query-service/) has been written to provide a REST API on top of the contract getters.

Other possible ways to fix the query issue:

- In release `0.6.0`, the Odra team plans to add a [WASM client](https://github.com/odradev/odra/issues/202) which will be auto-generated from the contract getters (or maybe straight from `struct` fields) and can be run in the browser.
- Emit events when the contract state changes. Those events can be indexed by some custom indexer, and then the frontend can query this indexer. It seems like [casper.cloud](https://cspr.cloud/) can become a general solution for this (it is probably one of the results of the [casper-dao-middleware](https://github.com/make-software/casper-dao-middleware) development).

### Codebase

The Contract on-chain and backend have their own Makefile in [voting](./voting/) directory. The commands provided there allow for building contracts, testing them with both Odra-mock and Casper VMs, building and running the query service, deploying contracts via Odra's `livenet` feature, and running E2E tests. For more details on deployment, please refer to the [Deploying the project section](#deploying-the-project).

Currently, the contract does not use any Events.

## Contract frontend

The frontend is a React application written in TypeScript with a very basic UI. This is my first experience with both React and TypeScript, so I suspect some things there are "pretty suboptimal" 🙃.

The application uses the `casper-js-sdk` to build and submit deploys when calling contract entry points. The code related to `casper-js-sdk is mostly` concentrated in the [GovernorClient.ts](./voting-frontend/src/GovernorClient.ts) file.

### Signing

It is possible to sign deploys using either the Casper Wallet browser extension or some known keys. For more information, refer to [CasperNetwork.ts](./voting-frontend/src/CasperNetwork.ts). To switch between the two methods of signing, use the `USE_CASPER_WALLET` constant in [Settings.ts](./voting-frontend/src/Settings.ts).

In [Utils.tsx](./voting-frontend/src/Utils.tsx), there are some hardcoded keys that are parsed from a Base64 encoded secret key. These keys were used for development and debugging. Adjust the module if needed.

It is also possible to parse keys from `.pem` files using `casper-js-sdk`.

## Deploying the project

The following section provides instructions for deploying the `Governor` contract on the testnet, creating a proposal that will call to a 3rd-party contract, and voting on it.

For additional instructions on how to run the project with a local private network, please refer to the [Switching the network](#switching-the-network) section. Note that the Casper Wallet browser extension can not connect to a custom network, and you will need either a Base64 encoded secret key or a secret key `.pem` file to sign deploys.

### 'Resetting' contract state

To reset the state of the contracts described below, simply redeploy them with the same account key. If you are using the [nctl-docker local network](./nctl-docker/), you can also use the [make command](./Makefile) to reset the private local network itself.

### Switching the network

Current setup is set to work with the `testnet`. To switch it to another network several changes are required:

- Change data [.env file in contract dir](./voting/contracts/.env) to use correct network name, node address and proper key. E.g., see [data for NCTL local network](./voting/contracts/.env.ln). `query-service` will copy and use this file if started via `make` command.
- Change [frontend settings](./voting-frontend/src/Settings.ts) accordingly. Important: for `NODE_URL` change only the part after the proxy url, e.g. for NTCL docker setup it will be `NODE_URL = 'http://localhost:3001/http://localhost:11101'`, and if you are using local network, then switch `USE_CASPER_WALLET` to `false`, as Casper Wallet extension can not connect to the custom network - deployments will be auto-signed with the key from [Utils.theKeys()](./voting-frontend/src/Utils.tsx#9)

### Testnet deploy

A live demo of all the steps below can be found [here](https://drive.google.com/drive/folders/1JlgKBKud909-tkmrEbGPnjzzlrFM68ex?usp=sharing). 

(Step 0: prepare a lot of terminals)

#### Step 1 - keys

Note: current state of the repo already has everything set up and keys should still have enough funds for experiments.

To deploy on the testnet, you will need keys. Keys can be created using the Casper Waller browser extension or the [casper-client](https://docs.casper.network/concepts/accounts-and-keys/). Keys created with the Casper client can be imported to the Casper Wallet browser extension.

To deploy the main contract with the current setup, you will need one secret key `.pem` file. It is possible to download keys from the Casper Wallet browser extension if needed.

After keys are created and added to the Casper Wallet, you can use the [faucet](https://testnet.cspr.live/tools/faucet) to request funds (only one request per address is allowed).

If needed, there are some funded keys available in the [testnet-keys directory](./testnet-keys/) of the repository.

It is a required step because the Casper Wallet will be used to sign deployments.

#### Step 2 - build and test

Build and test contracts.
```
cd voting
make test-w-casper
```

There should be no errors reported.

#### Step 3 - prepare environment

Setup Odra `livenet` environment.

Odra `livenet` feature allows developer to deploy and call contracts on the real network right from the Rust project. Path to the secret key, node url and network name are specified through the [.env file](./voting/contracts/.env). The file is currently set for testnet. `http://94.130.10.55:7777` is the RPC endpoint for some known public node running on testnet.

There is also an example of the Docker local network setup in the [.env.ln](./voting/contracts/.env.ln) file.

#### Step 4 - deploy governor

Deploy main contract - the `governor`, using Odra `livenet`.

You should be in the `voting` directory.

```
make deploy-via-livenet
```
If everything went OK, you should see output like this:

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

You can check the deployment hash and contract hash by visiting [https://testnet.cspr.live](https://testnet.cspr.live/). To find the deployment hash, look for messages like `WAIT: Waiting 15s for "..."` in the terminal.

During contract deployment, a `governor.json` file will be created in the `voting` directory. This file will contain the contract package hash, but not the contract hash (which will be explained later). The `query-service` will use this file so that the frontend can query it and determine which contract to call. This was primarily designed to speed up the development-debugging loop.

#### Step 5 - query service

Start query service.

You should be in the `voting` directory.

```
make run-query-service
```

`query-service` uses "getters" provided by the governor contract to query node, so environment setup for `livenet` Odra feature is also required here. When `make run-query-service` is executed, it copies `.env` file from the `contracts` directory into own `query-service` directory to keep them in sync. Then it starts web-service on port `8080`. Service is built with `actix` Rust library.

#### Step 6 - node proxy

Casper nodes require CORS. Developers had previously stated that starting from version `1.5`, CORS would be removed, and this was the case until `1.5.2`. The current testnet nodes run on `1.5.2`, so CORS is back on `testnet`.

The easiest way to deal with CORS I've found, at least for development, is to use a small TS server with `cors-anywhere`.


In the frontend settings, `NODE_URL` is specified as the URL for the proxy server with the node URL as an argument, see the [Settings.ts](./voting-frontend/src/Settings.ts) file. This allows to use node and contract clients provided by `casper-js-sdk` "as is" at the fronted w/o writing whole bunch of own code.

Perhaps this proxy can be merged with the `query-service`. But if Odra team releases the `WASM` contract client (see `Other possible variants` section of the [Odra cons section](#odra-cons)), there will be no need for `query-service`. For now, a standalone proxy is used.

Make sure to leave it running.

#### Step 7 - frontend

To start the frontend, open the terminal in the [voting-frontend directory](./voting-frontend/).

Then run

```
npm start
```

or, to prevent default browser launch

```
BROWSER="none" npm start 
```

The frontend app will start at `http://localhost:3000/`. You should see a couple of text fields and a form with a button there.

To use the frontend, the `governor` contract must be deployed and the `proxy` and `query-service` must be running. Additionally, to test the execution of arbitrary contracts, the `3d-party-contract` should be deployed. Let's move to it.

#### Step 8 - 3d-party-contract

A 3rd-party contract can be employed to test arbitrary contract execution as a result of voting. The contract is written in low-level Casper, and the full code is included for reference in the [main.rs file](./3d-party-contract/main.rs).

The contract stores a single variable on-chain that can be incremented by the `counter_inc` entry point. Amount to increment is passed through the entry point arguments.

Network interactions for deploying and querying this contract occur through shell scripts. These scripts use the `casper-client` CLI tool, which must be installed on your system.

Enter [3d-party-contract contract directory](./3d-party-contract/) in terminal.

Pick user key to work with and initiate environment. E.g. running `source ./test-net-debug-user-env` will export environment variables for the testnet node and secret key that have some casper funds.

Start deployment with

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

Now you can check the deploy hash either via `testnet.cspr.live` or by calling `./get-deploy.sh "THE_HASH"`.

With `testnet.cspr.live`, the current deployment status can be viewed in the UI. If using `./get-deploy.sh ...`, look for the `execution_results` key. The first `result` in the array will contain either `Success` or `Failure`. If it's `Failure`, the reason is usually included.

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
(Leave this terminal open - we will need it 🙂)

#### Step 9 - interact with the contract

After launching the frontend and deploying the `3d-party-contract`, the first thing you should do int he frontend UI is click the `Init` button. When you do this, the following happens:

- The app will request access to the Casper account and set the required data to the state. Casper Wallet will open and ask you to choose and connect your account. If you switch accounts, press `Init` again. Also, sometimes you may need to click `Init` again after refreshing the page to set the keys. This is a result of my lack of experience, and I couldn't figure out how to make it better in the time I had. Sometimes an error message may appear saying `CasperWalletProvider is not a function`. I'm not sure what causes this, but reloading the page without cache helps here.
- After the key is set, the application will make a request to the `query-service` to get the `package hash` of the governor contract. Then it will use `CasperClient` from `casper-js-sdk` to find the `contract hash` using the `package hash`. For some reason, the `Contract` client from `casper-js-sdk` needs the `contract hash` to call the contract and fails with the `package hash`. Casper natively supports contracts upgrade, and different versions of contracts with different hashes are stored under the single `package hash`. So the user can call a specific version of the contract using the `package hash` and version, or just call the latest version with just the `package hash`. Odra uses the `package hash` to call contracts, and `casper-client` can use both. However, `casper-js-sdk` needs the `contract hash`.

If everything went well, you should see the current public key hash, contract hash, and package hash appear above the form.

Now, let's move on to the proposals. You can create proposals using the form below.

You can specify the following from left to right (there is no validation done here):

- Proposal description
- Information required to call an external contract. This information will be stored on-chain as part of the proposal. The call will be executed if the proposal receives a majority of "YES" votes:
    - Package hash of the contract. Remember that Odra uses the `package hash` to call contracts. So, use the hash acquired during [3d-party-contract-deploy](#step-8---3d-party-contract).
    - Contract entry point to call. In case of `3d-party-contract`, it will be `counter_inc`.
    - Argument for the entry point. `counter_inc` accepts a number. I had no time to make it possible to pass any set of arguments supported by contract entry points through the form, so here arguments type is limited by `number`. There are no limits on the smart contract side - it will accept any serialized arguments that the frontend will supply to it. But currently, the frontend code has only arguments required for `counter_inc` hardcoded - see [submit function in NewProposal.tsx](./voting-frontend/src/NewProposal.tsx).

After filling out the form, click on `Add Proposal`. This will prompt you to sign the deployment with the Casper Wallet extension and then send the deployment to the node. You can monitor the process in the browser console, but an alert will also pop up when the deployment succeeds or fails.

Once the deployment is finished, you can refresh the page so that the app can query newly created proposals. Proposals should appear under the form. From here, you can vote on them (one vote per key) and then finalize the voting. If the proposal receives a majority of "YES" votes, the contract call stored inside the proposal will be executed. If it fails, you will see an error, and the proposal will remain unfinalized (with the current logic).

If a call to the `3d-party-contract` was used (and you don't have that many options with the current frontend 🙂) and it was executed during finalization, you can check if the call to the `3d-party-contract` was executed successfully.

Go back to `3d-party-contract` terminal and from there run:
  
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
        "parsed": x
      }
    }
  }
}
```

The `"parsed": X` is what should change by the amount you specified in the proposal form. During `3d-party-contract` deployment this value is set to `0`.

Now you can create more proposals, vote on them, and finalize them. Just remember to refresh the page, as it updates only with a fresh query response from the `query-service`. You may also need to click `Init` again after refreshing (sorry for the inconvenience).
