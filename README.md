# rust-lnd

The rust-lnd is the rust binding to the Lightning Network Daemon.

## Requirements

Go and lnd is required to generate the additional code and build the package.

## Updating

The file `protos/rpc.proto` should be synced with `https://github.com/lightningnetwork/lnd/blob/master/lnrpc/rpc.proto`.

## Usage

First, add the following to `Cargo.toml`:

```toml
[dependencies]
rust-lnd = "0.1"
grpc = "0.5.0"
```

Next, create lightning client object:

```rs
let lightning_client = {
    let certificate = lnd_rust::TLSCertificate::from_der_path("path/to/file.crt").unwrap();
    let config = Default::default();
    let socket = "127.0.0.1:9000".parse().unwrap();
    let grpc_client = certificate.create_client(&socket, "localhost", config);
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
