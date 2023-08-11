import {
  EventStream,
  EventName,
  CasperServiceByJsonRPC
} from "casper-js-sdk";

import { Event, Parser, fetchContractSchemasBytes, parseSchemasFromBytes, parseEventNameAndData } from "@make-software/ces-js-parser";
import { ExampleContractClient } from "./ExampleContractClient";

class SomeEvent {
  // readonly message: string

  private constructor(public message: string) { }

  public static fromEvent(event: Event): SomeEvent | undefined {
    if (event.name != "SomeEvent" || event.data["message"] == undefined) {
      return undefined
    }
    return new SomeEvent(event.data.message.data)
  }
}

export class EventHandler {

  private constructor(readonly eventStream: EventStream, readonly ecClient: ExampleContractClient) { }

  public static async create(ecClient: ExampleContractClient) {
    const es = new EventStream(ecClient.nodeEventsUrl);
    return new EventHandler(es, ecClient)
  }

  public async startListening(processEvent: (event: SomeEvent | undefined) => void) {
    const rpcClient = new CasperServiceByJsonRPC(this.ecClient.nodeRpcUrl)
    const parser = await Parser.create(
      rpcClient,
      [normalizeHash(this.ecClient.getContractHash())]
    )
    this.eventStream.start()

    this.eventStream.subscribe(EventName.DeployProcessed, async (event) => {
      const executionResult = event.body.DeployProcessed.execution_result
      try {
        const parseResults = parser.parseExecutionResult(executionResult);
        if (parseResults.length > 0) {
          // TODO: probably persisting of last received pr.id is required, coz
          // in case of dApp restart event listener can receive some old events
          parseResults.map(pr => SomeEvent.fromEvent(pr.event)).forEach(processEvent);
        }
      } catch (e) {
        console.error(`EventHandler: Failed to parse deploy event: ${e}`) // TODO: more info in error
      }
      
    })
  }

  public async stopListening() {
    this.eventStream.stop()
  }
}

function normalizeHash(contractHash: string): string {
  return contractHash.startsWith("hash-") ? contractHash.slice(5) : contractHash
}
