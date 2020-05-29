use tiberius::{AuthMethod, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut builder = Client::builder();
    builder.host("db");
    builder.port(1433);
    builder.database("test");
    builder.authentication(AuthMethod::sql_server("SA", "P@ssw0rd!"));
    builder.trust_cert();

    let mut client = builder.build().await?;
    let results = client
        .simple_query("select id, name from Users;")
        .await?
        .into_results()
        .await?;

    for val in results.iter() {
        // 取得した件数分ループする
        for inner in val.iter() {
            // id列の情報を取得
            if let Some(id) = inner.get::<i32, _>("id") {
                print!("id = {} ", id);
            }
            // name列の情報を取得
            if let Some(name) = inner.get::<&str, _>("name") {
                println!("name = {}", name);
            }
        }
    }
    Ok(())
}