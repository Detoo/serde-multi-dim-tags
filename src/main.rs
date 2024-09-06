use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "topic", rename_all = "camelCase")]
enum MessageTopic {
    Market(MessageEvent<MarketData>),
    Portfolio(MessageEvent<PortfolioData>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
enum MessageEvent<T> {
    Snapshot(T),
    Update(T),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MarketData {
    markets: Vec<MarketStats>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MarketStats {
    ticker: String,
    price: f64,
    volume: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PortfolioData {
    open_orders: Vec<OpenOrder>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct OpenOrder {
    id: i16,
    side: Side,
    ticker: String,
    limit_price: f64,
}

fn main() {
    let raw_messages = vec![
        r#"{"topic":"market","event":"snapshot","markets":[{"ticker":"BTC/USD","price":1000000,"volume":1000},{"ticker":"ETH/USD","price":10000,"volume":10000}]}"#,
        r#"{"topic":"portfolio","event":"snapshot","open_orders":[{"id":0,"side":"sell","ticker":"BTC/USD","limit_price":1100000}]}"#,
        r#"{"topic":"market","event":"update","markets":[{"ticker":"BTC/USD","price":1000001,"volume":1001}]}"#,
        r#"{"topic":"portfolio","event":"update","open_orders":[{"id":1,"side":"buy","ticker":"ETH/USD","limit_price":9000}]}"#,
    ];

    let mut message_stream = raw_messages
        .into_iter()
        .map(|message| serde_json::from_str::<MessageTopic>(message));

    while let Some(message) = message_stream.next() {
        match message {
            Ok(MessageTopic::Market(MessageEvent::Snapshot(market_data))) => {
                println!("Got market snapshot: {:?}", market_data);
            }
            Ok(MessageTopic::Market(MessageEvent::Update(market_data))) => {
                println!("Got market update: {:?}", market_data);
            }
            Ok(MessageTopic::Portfolio(MessageEvent::Snapshot(portfolio_data))) => {
                println!("Got portfolio snapshot: {:?}", portfolio_data);
            }
            Ok(MessageTopic::Portfolio(MessageEvent::Update(portfolio_data))) => {
                println!("Got portfolio update: {:?}", portfolio_data);
            }
            Err(result) => println!("Unexpected message: {:?}", result),
            // TODO Uncomment this if there are other message variants you want to ignore
            // _ => {
            //     println!("Ignoring message: {:?}", message);
            // }
        }
    }
}
