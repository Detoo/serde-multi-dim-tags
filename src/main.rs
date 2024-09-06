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
    Snapshot(Message<T>),
    Update(Message<T>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Message<T> {
    nonce: i64,
    #[serde(flatten)]
    payload: T,
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
    let message = MessageTopic::Market(MessageEvent::Snapshot(Message {
        nonce: 0,
        payload: MarketData {
            markets: vec![MarketStats {
                ticker: "BTC/USD".to_string(),
                price: 1_000_000.0,
                volume: 1_000.0,
            }],
        },
    }));

    let serialized_message = serde_json::to_string(&message).unwrap();
    println!("{}", serialized_message);

    assert_eq!(
        serde_json::from_str::<MessageTopic>(&serialized_message).unwrap(),
        message
    );
}
