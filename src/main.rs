use clap::{Parser};
use std::{fs, net::IpAddr, path::PathBuf, time::Duration};
use tokio::{net::TcpStream, time::timeout};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    ip: IpAddr,
    #[arg(short, long)]
    key_path: PathBuf,
    #[arg(short, long, default_value = "dev")]
    user: String,
}

async fn ping_server(addr: &IpAddr) -> bool {
    match timeout(Duration::from_secs(3), TcpStream::connect((*addr, 80))).await {
        Ok(Ok(_stream)) => true,
        _ => false,
    }
}

#[derive(Debug)]
enum KeyFileError {
    NotFound,
    IsDirectory,
    NoReadPermissions(String),
    ParseMetadata(String),
    InvalidKey,
}

fn validate_key_file(path: &PathBuf) -> Result<(), KeyFileError> {
    if !path.exists() {
        return Err(KeyFileError::NotFound);
    }

    if path.is_dir() {
        return Err(KeyFileError::IsDirectory);
    };

    let contents = fs::read_to_string(path).map_err(|e| KeyFileError::NoReadPermissions(e.to_string()))?;

    println!("{}", contents);

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if ping_server(&args.ip).await {
        println!("Server {} is reachable for user {}!", args.ip, args.user);
    } else {
        eprintln!("Could not reach server.");
    }

    validate_key_file(&args.key_path).expect("err");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_ping_timeout() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1));
        let result = ping_server(&ip).await;
        assert_eq!(result, false, "Should return false");
    }
}
