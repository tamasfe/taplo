import loadTaplo from "../../../crates/taplo-wasm/Cargo.toml";
import { convertEnv, Environment, Lsp, prepareEnv } from "@taplo/core";

export interface RpcMessage {
  jsonrpc: "2.0";
  method?: string;
  id?: string | number;
  params?: any;
  result?: any;
  error?: any;
}

export interface LspInterface {
  /**
   * Handler for RPC messages set from the LSP server.
   */
  onMessage: (message: RpcMessage) => void;
}

export class TaploLsp {
  private static taplo: any | undefined;
  private static initializing: boolean = false;

  private constructor(private env: Environment, private lspInner: any) {
    if (!TaploLsp.initializing) {
      throw new Error(
        `an instance of Taplo can only be created by calling the "initialize" static method`
      );
    }
  }

  public static async initialize(
    env: Environment,
    lspInterface: LspInterface
  ): Promise<TaploLsp> {
    if (typeof TaploLsp.taplo === "undefined") {
      TaploLsp.taplo = await loadTaplo();
    }
    TaploLsp.taplo.initialize();

    prepareEnv(env);

    TaploLsp.initializing = true;
    const t = new TaploLsp(
      env,
      TaploLsp.taplo.create_lsp(convertEnv(env), {
        js_on_message: lspInterface.onMessage,
      })
    );
    TaploLsp.initializing = false;

    return t;
  }

  public send(message: RpcMessage) {
    this.lspInner.send(message);
  }

  public dispose() {
    this.lspInner.free();
  }
}
