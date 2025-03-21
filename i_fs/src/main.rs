use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open("foo.txt").await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    println!("file content is: {}", contents);

    let mut outfile = fs::File::create("bar.txt").await?;
    outfile.write_all(contents.as_bytes()).await?;

    Ok(())
}
