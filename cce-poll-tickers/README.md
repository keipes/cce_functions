# CCE: Poll Tickers
This lambda function retrieves the ticker-cik mapping from the SEC, converts it to JSON, and uploads the result to S3.

## Development

Dev server uses [cargo-lambda/cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)
Docker build uses [rustserverless/lambda-rust](https://hub.docker.com/r/rustserverless/lambda-rust)

### Serve
Start a local lambda runtime environment: `cargo lambda start`

### Invoke
Send a test payload to the local lambda: `cargo lambda invoke --data-file sample_event.json`

### Build

```
docker run --rm --platform linux/arm64/v8 -v ${PWD}:/code -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/git:/root/.cargo/git rustserverless/lambda-rust:latest-arm64
```
