import { invoke } from "@tauri-apps/api";

// Please see src-tauri/src/data/models.rs for original implementation of models.

export interface ClientInfo {
  url: string;
  authHeader: string;
}

export interface EndpointMap {
  [name: string]: Endpoint;
}

export interface Endpoint {
  plugins: Plugin[];
}

export interface Plugin {
  method: string;
  path: string;
  description?: string;
  operationId?: string;
  parameters?: any;
  responses?: any;
  summary: string;
  requestBody?: any;
}

export interface SchemaMap {
  [name: string]: Schema;
}

export interface Schema {
  name: string;
  description?: string;
  properties?: any;
  enum?: string[];
  type: string;
  schemaIds: string[];
}

// Invokes commands from src-tauri/src/server/handlers.rs
export class Invoker {
  constructor() {}

  static get Key() {
    return "Invoker";
  }

  async client_info(): Promise<ClientInfo> {
    return (await invoke("get_client_info")) as ClientInfo;
  }

  async endpoints(): Promise<EndpointMap> {
    return (await invoke("get_endpoints")) as EndpointMap;
  }

  async endpoint_by_name(name: string): Promise<Endpoint> {
    return (await invoke("get_endpoint", { name })) as Endpoint;
  }

  async schemas(): Promise<SchemaMap> {
    return (await invoke("get_schemas")) as SchemaMap;
  }

  async schema_by_name(name: string): Promise<Schema> {
    return (await invoke("get_schema", { name })) as Schema;
  }

  async send_request(method: string, path: string, body: any): Promise<any> {
    return (await invoke("send_request", {
      method,
      path,
      body: body != null ? JSON.stringify(JSON.parse(body)) : null,
    })) as any;
  }
}
