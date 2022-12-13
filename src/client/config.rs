use tokio::fs;

pub struct ClientConfig {

	/// The name of IP address of the name server to query.
	pub nameserver: String,
}

impl ClientConfig {

	pub async fn new() -> std::io::Result<ClientConfig> {

		let nameserver = Self::nameserver().await?;

		Ok(ClientConfig {
			nameserver,
		})
	}

	async fn nameserver() -> std::io::Result<String> {

		let resolv = fs::read_to_string("/etc/resolv.conf").await?;

		Ok(resolv
			.lines()
			.filter(|l| l.starts_with("nameserver "))
			.map(|l| l.trim().replace("nameserver ", ""))
			.last()
			.unwrap_or(String::from("8.8.8.8"))
		)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	// TODO: How do we stub in Rust?
	#[tokio::test]
	async fn it_() {

		let cfg = ClientConfig::new().await.unwrap();

		assert_eq!(cfg.nameserver, "192.168.0.1");
	}
}
