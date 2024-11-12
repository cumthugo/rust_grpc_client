use rand::Rng;

pub mod hello {
    tonic::include_proto!("ford.ivi.h625");
}

use hello::FooRequest;

#[derive(Default)]
pub struct MyGreeter {}

// 客户端代码
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 客户端测试
    let mut client = hello::greeter_client::GreeterClient::connect("http://127.0.0.1:50051").await?;

    // 随机选择一个字符串出来
    let names = ["one", "two", "three"];

    let mut rng = rand::thread_rng();

    let random_name = names[rng.gen_range(0..names.len())];

    let request = tonic::Request::new(FooRequest {
        name: random_name.into(),
    });

    let response = client.foo(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}
