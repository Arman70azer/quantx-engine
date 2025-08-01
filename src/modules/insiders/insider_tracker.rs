
use std::error::Error;
use scraper::{Html, Selector};
use reqwest::blocking::get;

#[derive(Debug)]
pub struct InsiderTrade {
    pub name: String,
    pub title: String,
    pub trade_date: String,
    pub transaction_type: String,
    pub price: String,
    pub qty: String,
}

fn normalize_name(raw: &str) -> String {
    let parts: Vec<&str> = raw.trim().split_whitespace().collect();
    if parts.len() == 2 {
        format!("{} {}", parts[1], parts[0])
    } else {
        raw.to_string()
    }
}

fn clean_number(raw: &str, qty: bool) -> String {
    let mut cleaned = raw.replace(',', "").replace('$', "").trim().to_string();
    if qty {
        cleaned = cleaned.replace('-', "").trim().to_string();
    }
    cleaned
}


impl InsiderTrade {
    pub fn fetch_insiders_trades(action: &str) -> Result<Vec<InsiderTrade>, Box<dyn Error>> {
        let url = format!("http://openinsider.com/screener?s={}&o=&pl=&ph=&ll=&lh=&fd=730&fdr=&td=0&tdr=&fdlyl=&fdlyh=&daysago=&xp=1&xs=1&vl=&vh=&ocl=&och=&sic1=-1&sicl=100&sich=9999&grp=0&nfl=&nfh=&nil=&nih=&nol=&noh=&v2l=&v2h=&oc2l=&oc2h=&sortcol=0&cnt=100&page=1", action);


        let body = get(url)?.text()?;
        let document = Html::parse_document(&body);

        let row_selector = Selector::parse("table.tinytable tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        let mut trades = Vec::new();

        for (i, row) in document.select(&row_selector).enumerate() {
            if i == 0 {
                continue; // skip header
            }

            let tds: Vec<_> = row.select(&td_selector).collect();

            if tds.len() < 10 {
                continue;
            }

            let name = if let Some(a) = tds[1].select(&link_selector).next() {
                a.text().collect::<String>().trim().to_string()
            } else {
                tds[1].text().collect::<String>().trim().to_string()
            };

            let title = tds[2].text().collect::<String>().trim().to_string();
            let trade_date = tds[4].text().collect::<String>().trim().to_string();
            let transaction_raw = tds[6].text().collect::<String>().to_lowercase();
            let transaction_type = if transaction_raw.contains("sale") {
                "sell"
            } else {
                "buy"
            };

            let price = clean_number(&tds[7].text().collect::<String>().trim(), false);
            let qty = clean_number(&tds[8].text().collect::<String>().trim(), true);

            trades.push(InsiderTrade {
                name: normalize_name(&name),
                title,
                trade_date,
                transaction_type: transaction_type.to_string(),
                price,
                qty,
            });
        }

        Ok(trades)
    }

    pub fn println(&self) {
        println!(
            "Nom: {}, Titre: {}, Date: {}, Type: {}, Prix: {}, Quantité: {}",
            self.name, self.title, self.trade_date, self.transaction_type, self.price, self.qty
        );
    }
}
