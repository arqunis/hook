use hook::hook;

#[hook]
async fn ping() {
    println!("pong!");
}
