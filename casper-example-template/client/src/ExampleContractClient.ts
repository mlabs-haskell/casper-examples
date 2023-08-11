import {
  CasperClient,
  Contracts,
  RuntimeArgs,
  CLPublicKey,
  DeployUtil,
  Keys,
  GetDeployResult,
  CLValueBuilder,
  CLMap,
  CLString,
  CLBool,

} from "casper-js-sdk";


type DeployHash = string

export class ExampleContractClient {

  readonly contractKey = "add_with_registry_contract_key";
  private casperClient: CasperClient;

  public contractClient: Contracts.Contract;

  private contractHash: string | undefined = undefined;

  constructor(
    readonly nodeRpcUrl: string,
    readonly nodeEventsUrl: string,
    readonly chainName: string,
    readonly contractAccount: CLPublicKey
  ) {
    this.casperClient = new CasperClient(nodeRpcUrl);
    this.contractClient = new Contracts.Contract(this.casperClient);
  }

  public mkInstallDeploy(
    wasm: Uint8Array,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[],
  ): DeployUtil.Deploy {
    this.contractClient.setContractHash
    return this.contractClient.install(
      wasm,
      RuntimeArgs.fromMap({}),
      paymentAmount,
      deploySender,
      this.chainName,
      keys
    )
  }

  async findContractHash(): Promise<string | undefined> {
    const rootHash = await this.casperClient.nodeClient.getStateRootHash()
    const accountHash = this.contractAccount.toAccountHashStr()
    const state = await this.casperClient.nodeClient
      .getBlockState(rootHash, accountHash, [])
    return state
      .Account
      ?.namedKeys
      .find(key => key.name === this.contractKey)
      ?.key
  }

  // Attempts to find contract hash and initialize client with it. Will throw Error if contract key caould not be found.
  public async initWithContractHash(): Promise<void> {
    const hash = await this.findContractHash()
    if (!hash) {
      throw new Error(`Contract hash not found under expected key "${this.contractKey}" in Account. Is contract deployed?`)
    }
    this.contractHash = hash!
    this.contractClient.setContractHash(this.contractHash)
  }

  public getContractHash() {
    const hash = this.contractHash
    if (!hash) {
      throw new Error("Contract hash not set. Shoud use `initWithContractHash` to be able to call get.")
    }
    return hash!
  }

  public async installOnChain(
    wasm: Uint8Array,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[]
  ): Promise<[DeployUtil.Deploy, string]> {
    const installDeploy = this.mkInstallDeploy(wasm, paymentAmount, deploySender, keys)
    return this.casperClient.putDeploy(installDeploy)
      .then(deployHash => { return [installDeploy, deployHash] })
  }

  public async awaitDeploy(
    deploy: DeployUtil.Deploy,
    timeout?: number
  ): Promise<GetDeployResult> {
    return this.casperClient.nodeClient.waitForDeploy(deploy, timeout)
  }

  public async register(
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[],): Promise<[DeployUtil.Deploy, string]> {
    const deploy = this.contractClient.callEntrypoint(
      "register_user_key",
      RuntimeArgs.fromMap({}),
      deploySender,
      this.chainName,
      paymentAmount,
      keys
    )
    return this.casperClient.putDeploy(deploy)
      .then(deployHash => { return [deploy, deployHash] })
  }

  public async append(
    phrase: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[]
  ): Promise<[DeployUtil.Deploy, string]> {
    const deploy = this.contractClient.callEntrypoint(
      "append_phrase",
      RuntimeArgs.fromMap({ what_to_append: CLValueBuilder.string(phrase) }),
      deploySender,
      this.chainName,
      paymentAmount,
      keys
    )
    return this.casperClient.putDeploy(deploy)
      .then(deployHash => { return [deploy, deployHash] })
  }

  public async emitEvent(
    message: string,
    paymentAmount: string,
    deploySender: CLPublicKey,
    keys: Keys.AsymmetricKey[],): Promise<[DeployUtil.Deploy, string]> {
    const deploy = this.contractClient.callEntrypoint(
      "emit_event",
      // TODO: write note on how to put sonstants in rust side, coz you can't put
      // some-event-message as record field, 
      // and "some_event_message" unless you use NamedArg 
      RuntimeArgs.fromMap({ some_event_message: CLValueBuilder.string(message) }),
      deploySender,
      this.chainName,
      paymentAmount,
      keys
    )
    return this.casperClient.putDeploy(deploy)
      .then(deployHash => { return [deploy, deployHash] })
  }

  public async queyPhrase(): Promise<string> {
    return this.contractClient.queryContractData(["accumulator_value"])
  }

  public async queyRegisteredAccounts(): Promise<Map<string,boolean>> {
    const clMap = await this.contractClient.queryContractDictionary(
      "contract_dict",
      "registry_map"
    )
    return Contracts.fromCLMap(clMap.data)
  }

  public static isDeploySuccesfull(deployResult: GetDeployResult): boolean {
    if (deployResult.execution_results[0].result.Success) {
      return true
    } else {
      return false
    }
  }
}