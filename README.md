Defining a websocket handler with more than one Path parameter is not working

This sets up a poem server. A client task sleeps and then attempts to call the websocket handler. 

This handler has 2 path parameters, and always returns 400 Request. 

The body is `invalid path params` with no further information.

Full log is:

`Connect Http Error: Status 400, Body Some("invalid path params")`