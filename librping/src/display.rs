use tabled::Table;
use tabled::settings::Style;

use crate::types::Dns;

/// Displays a vector of DNS records in a modern table format on the console.
///
/// ### Parameters
/// - `dns`: A vector of `Dns` structs to display.
///
/// ### Example
/// ```rust
/// use crate::types::Dns;
/// let dns_list = vec![Dns::new("host1".to_string(), "192.168.1.1".to_string())];
/// display_dns(dns_list);
/// ```
pub fn display_dns(dns: Vec<Dns>) {
    let mut table: Table = Table::new(dns);

    table.with(Style::modern());

    println!("{}", table);
}
