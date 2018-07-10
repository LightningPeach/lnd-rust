# lnd-rust

The lnd-rust is the rust binding to the Lightning Network Daemon.

## Requirements

Go and lnd are required to generate the additional code and build the package.

## Updating

The file `protos/rpc.proto` should be synced with `https://github.com/lightningnetwork/lnd/blob/master/lnrpc/rpc.proto`.

Tested with the Lightning Network Daemon version "0.4.2".

## Usage

First, add the following to `Cargo.toml`:

```toml
[dependencies]
lnd-rust = "0.1"
grpc = "0.5.0"
```

Next, create lightning client object:

```rs
let lightning_client = {
    let certificate = lnd_rust::TLSCertificate::from_der_path("path/to/file.crt").unwrap();
    let config = Default::default();
    let socket = "127.0.0.1:9000".parse().unwrap();
    let tls = certificate.into_tls("localhost").unwrap();
    let grpc_client = grpc::Client::new_expl(&socket_addr, "localhost", tls, config).unwrap();
    lnd_rust::rpc_grpc::LightningClient::with_client(grpc_client)
};
```

Now, it is possible to make requests:

```rs
let request = lnd_rust::rpc::GetInfoRequest::new();
let options = grpc::RequestOptions::new();
let response = lightning_client.get_info(options, request);
```

See `src/bin/main.rs` for details.
