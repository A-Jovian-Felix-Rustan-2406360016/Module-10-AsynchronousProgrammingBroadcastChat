use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    // TODO: For a hint, see the description of the task below.
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_stream.send(Message::text(text)).await?;
                    }
                    Ok(None) => {
                        break;
                    }
                    Err(e) => {
                        eprintln!("Gagal membaca dari stdin: {e}");
                        break;
                    }
                }
            }
            
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            println!("{}", msg.as_text().unwrap_or_default());
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error menerima pesan dari server: {e}");
                        break;
                    }
                    None => {
                        println!("Koneksi ke server terputus.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())

}