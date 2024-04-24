use rand::Rng;

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::HelloRequest;

#[derive(Default)]
pub struct MyGreeter {}

// 客户端代码
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 客户端测试
    let mut client = hello::greeter_client::GreeterClient::connect("http://[::1]:50051").await?;

    // 随机选择一个字符串出来
    let names = ["张三", "李四", "王五"];

    let mut rng = rand::thread_rng();

    let random_name = names[rng.gen_range(0..names.len())];

    let request = tonic::Request::new(HelloRequest {
        name: random_name.into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}
