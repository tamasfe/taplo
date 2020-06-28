# LSP Async Stub

This library provides utilities and a basic server stub for asynchronous LSP communication with JSON RPC.

The server stub expects async handlers that will be called for each message along with the parameters
in the message and a context for shared data, cancellation and sending further requests/notifications back to the client.
It also handles invalid messages, initialization and teardown according to the LSP spec.

I haven't published it as a separate crate only because I didn't bother to document and test it, otherwise it is working fine so far.

If you wish to turn it into a proper library, feel free to make your own fork, open an issue, or pull request about it!