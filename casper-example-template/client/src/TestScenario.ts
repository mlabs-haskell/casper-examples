
//TODO:
// - move gas costs to constants
// - think of better client API

import {
  CLBool,
  CLMap,
  CLString,
  CLValueBuilder,
  DeployUtil,
  Keys,
  Contracts


} from "casper-js-sdk";

import { readKeys, readWasm } from "./Utils";
import { ExampleContractClient } from "./ExampleContractClient";
import { EventHandler } from "./ContractEvents";
// import { fromCLMap } from "casper-js-sdk/dist/lib/Contracts";


enum Network {
  // MAINNET = "mainnet",
  TESTNET = "testnet",
  PRIVATE = "private"
}

const currentNetwork = Network.PRIVATE;

const TESTNET_KEYS = "/path/to/keys";

function setupEnv(network: Network): [string, Keys.AsymmetricKey, string, string] {
  switch (network.valueOf()) {
    case Network.PRIVATE:
      return [
        "casper-net-1",
        readKeys("../nctl-docker/users/user-10"),
        "http://localhost:11101/rpc"
        , "http://localhost:18101/events/main"
      ]

    case Network.TESTNET:
      return [
        "casper-test",
        readKeys(TESTNET_KEYS),
        "http://94.130.10.55:7777/rpc",
        "http://94.130.10.55:9999/events/main"
      ]

    default:
      throw new Error("Unknown network: " + currentNetwork)
  }
}

const [network, keys, nodeRpc, nodeEvents] = setupEnv(currentNetwork)

const exampleContractClient = new ExampleContractClient(nodeRpc, nodeEvents, network, keys.publicKey)


const wasmPath = "./wasm/contract.wasm"

// can be found from regression cost test in contract repo
const contractInstallCost = "50334128500"

async function runScenario() {
  const eventHandler = await EventHandler.create(exampleContractClient)

  const contractHash = await exampleContractClient.findContractHash()
  console.log({ contractHash: contractHash })

  if (!contractHash) {
    await installContract()
  } else {
    console.log("Contract already installed. Proceeding to endpoints calls.")
  }

  console.log("Initializing client with hash of deployed contract")
  await exampleContractClient.initWithContractHash()

  console.log("Start listening events")
  eventHandler.startListening(ev => {
    console.log(JSON.stringify(ev))
  })


  await registerAndWaitExecuted()
  await appendAndWaitExecuted("Append B")

  const registrations = await exampleContractClient.queyRegisteredAccounts()
  console.log(`Registrations:\n${JSON.stringify(Object.fromEntries(registrations))}`)


  await emitEvent("test-message-2")
  await emitEvent("test-message-3")

  const currentPhrase = await exampleContractClient.queyPhrase()
  console.log(`Current phrase: ${currentPhrase}`)

}

async function appendAndWaitExecuted(phrase: string) {
  let [appendDeploy, appendDeployHash] = await exampleContractClient.append(
    phrase,
    "502402510",
    keys.publicKey,
    [keys]
  )
  console.log(`Awaiting append deploy executed. Hash: ${appendDeployHash}`)
  const appendDeployResult = await exampleContractClient.awaitDeploy(appendDeploy)
  if (!ExampleContractClient.isDeploySuccesfull(appendDeployResult)) {
    const cause = appendDeployResult.execution_results[0].result.Failure?.error_message
    console.log(`Append failed: ${cause}`)
  } else {
    console.log(`Appended successfully!`)
  }
}

async function registerAndWaitExecuted() {
  let [regDeploy, regDeployHash] = await exampleContractClient.register(
    "502402510",
    keys.publicKey,
    [keys]
  )
  console.log(`Awaiting register deploy executed. Hash: ${regDeployHash}`)
  const regDeployResult = await exampleContractClient.awaitDeploy(regDeploy)
  console.log(regDeployResult.execution_results[0].result)
}

async function emitEvent(message: string) {
  console.log("Calling event")
  let [eventDeploy, eventDeployHash] = await exampleContractClient.emitEvent(
    message,
    "502402510",
    keys.publicKey,
    [keys]
  )
  console.log("Awaiting event deploy executed. Hash: " + eventDeployHash)
  await exampleContractClient.awaitDeploy(eventDeploy)
  // console.log(eventDeployResult.execution_results[0].result)
}

async function installContract() {
  const wasm = readWasm(wasmPath)
  const [installDeploy, deployHash] = await exampleContractClient.installOnChain(
    wasm,
    contractInstallCost,
    keys.publicKey,
    [keys]
  )

  console.log({ deployHash: deployHash })

  console.log("Awaiting install deploy ready...")
  const installDeployResult = await exampleContractClient.awaitDeploy(installDeploy)

  if (!ExampleContractClient.isDeploySuccesfull(installDeployResult)) {
    console.log({ installDeployResult: installDeployResult.execution_results[0].result })
    const cause = installDeployResult.execution_results[0].result.Failure?.error_message
    throw new Error("Install deploy failed: " + cause)
  }
  console.log("Contract installed")
}


runScenario().then(res => {
  console.log("--- Result ---")
  console.log(res)
}
).catch(e => console.log("Error calling scenario: " + e))