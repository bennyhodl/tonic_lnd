// This example only fetches and prints the channel insights to the standard output similarly to
// `frcli insights`.
//
// This program accepts three arguments: address, cert file, macaroon file
// The address must start with `https://`!

#[tokio::main]
async fn main() {
    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address = args.next().expect("missing arguments: address, cert file, macaroon file");
    let cert_file = args.next().expect("missing arguments: cert file, macaroon file");
    let macaroon_file = args.next().expect("missing argument: macaroon file");
    let address = address.into_string().expect("address is not UTF-8");

    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let insights = client.faraday()
        // All calls require at least empty parameter
        .channel_insights(tonic_lnd::frdrpc::ChannelInsightsRequest {})
        .await
        .expect("failed to get channel insights").into_inner();

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", insights);
}
