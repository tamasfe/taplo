export namespace Server {
  interface ServerNotifications {
    "taplo/messageWithOutput": {
      params: {
        kind: "info" | "warn" | "error";
        message: string;
      };
    };
    "taplo/didChangeSchemaAssociation": {
      params: {
        documentUri: string;
        schemaUri?: string | null;
        meta?: any;
      };
    };
  }

  export type NotificationMethod = keyof ServerNotifications;

  export type NotificationParams<T extends keyof ServerNotifications> =
    ServerNotifications[T] extends NotificationDescription
      ? ServerNotifications[T]["params"]
      : never;
}

export namespace Client {
  interface ClientNotifications {
    "taplo/associateSchema": {
      params: {
        document_uri?: string | null;
        schema_uri: string;
        rule: AssociationRule;
        priority?: number | null;
        meta?: any;
      };
    };
  }

  interface ClientRequests {
    "taplo/convertToJson": {
      params: {
        text: string;
      };
      response: {
        text?: string | null;
        error?: string | null;
      };
    };
    "taplo/convertToToml": {
      params: {
        text: string;
      };
      response: {
        text?: string | null;
        error?: string | null;
      };
    };
    "taplo/listSchemas": {
      params: {
        documentUri: string;
      };
      response: {
        schemas: Array<SchemaInfo>;
      };
    };
    "taplo/associatedSchema": {
      params: {
        documentUri: string;
      };
      response: {
        schema?: SchemaInfo | null;
      };
    };
  }

  export type NotificationMethod = keyof ClientNotifications;

  export type NotificationParams<T extends keyof ClientNotifications> =
    ClientNotifications[T] extends NotificationDescription
      ? ClientNotifications[T]["params"]
      : never;

  export type RequestMethod = keyof ClientRequests;

  export type RequestParams<T extends keyof ClientRequests> =
    ClientRequests[T] extends RequestDescription
      ? ClientRequests[T]["params"]
      : never;

  export type RequestResponse<T extends keyof ClientRequests> =
    ClientRequests[T] extends RequestDescription
      ? ClientRequests[T]["response"]
      : never;
}

interface NotificationDescription {
  readonly params: any;
}

interface RequestDescription {
  readonly params: any;
  readonly response: any;
}

export type AssociationRule =
  | { glob: string }
  | { regex: string }
  | { url: string };

export interface SchemaInfo {
  url: string;
  meta: any;
}
