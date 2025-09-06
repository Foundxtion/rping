use tabled::Table;
use tabled::settings::Style;

use crate::types::Dns;

pub fn display_dns(dns: Vec<Dns>) {
    let mut table: Table = Table::new(dns);

    table.with(Style::modern());

    println!("{}", table);
}
