use bollard::errors::Error;
use bollard::image::ImportImageOptions;

use futures_util::stream::StreamExt;
use std::default::Default;
use tokio::fs::File;
use tokio_util::codec;

#[tokio::main]
async fn main() {
    let mut docker = bollard::Docker::connect_with_local_defaults().unwrap();
    let options = ImportImageOptions {
        ..Default::default()
    };

    async move {
        let mut file = File::open("looper.tar.gz").await.unwrap();

        let mut byte_stream = codec::FramedRead::new(file, codec::BytesCodec::new()).map(|r| {
            let bytes = r.unwrap().freeze();
            Ok::<_, Error>(bytes)
        });

        let mut bytes = Vec::new();

        while let Some(byte) = byte_stream.next().await {
            bytes.extend_from_slice(&byte.unwrap());
        }

        let mut stream = docker.import_image(
            ImportImageOptions {
                ..Default::default()
            },
            bytes.into(),
            None,
        );

        while let Some(response) = stream.next().await {
            println!("{:?}", response.unwrap());
        }
    }
    .await;
}
