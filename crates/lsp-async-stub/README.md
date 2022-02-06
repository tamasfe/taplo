# LSP Async Stub

This library provides utilities and a basic server stub for asynchronous LSP communication with JSON RPC.

Used mainly by `taplo-lsp`.

The server stub expects async handlers that will be called for each message along with the parameters
in the message and a context for shared data, cancellation and sending further requests/notifications back to the client.
It also handles invalid messages, initialization and teardown according to the LSP spec.
