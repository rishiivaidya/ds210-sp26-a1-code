use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};
use interface::RPCInterfaceClient;
use tarpc::context::Context;

pub async fn run_hello(rpc_client: &RPCInterfaceClient) {
    let result = rpc_client.hello(Context::current()).await.unwrap();
    println!("The server says: `{}`", result);
}

pub async fn run_slow_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using slow_rpc");
    let dataset = rpc_client.slow_rpc(Context::current()).await.unwrap();
    return compute_query_on_dataset(&dataset, &query); //use query

} //slow_rpc sends back the whole dataset, so the client still has to do the actual query work itself

pub async fn run_fast_rpc(rpc_client: &RPCInterfaceClient, query: Query) -> Dataset {
    println!("using fast_rpc");
    return rpc_client.fast_rpc(Context::current(), query).await.unwrap();
}
//implemented run_fast_rpc using .await.unwrap() model from earlier code