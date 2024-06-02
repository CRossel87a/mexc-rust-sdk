

#[cfg(test)]
mod tests {
    use crate::orders::OrderSide;
    use crate::orders::OrderType;
    use crate::orders::OrderReceipe;
    use crate::{utils::unlock_keys, Mexc};


    #[tokio::test]
    pub async fn test_get_server_time() {

        let client = Mexc::new(None,None,None).unwrap();

        let time = client.get_server_time().await.unwrap();
        dbg!(time);
    }

    #[tokio::test]
    pub async fn test_ping() {

        let client = Mexc::new(None,None,None).unwrap();

        let dur = client.ping().await.unwrap();
        dbg!(dur);
    }

    
    #[tokio::test]
    pub async fn test_symbol_info() {
        let client = Mexc::new(None,None,None).unwrap();
        let info = client.symbol_info("PLSUSDT").await.unwrap();
        dbg!(info);
    }

    #[tokio::test]
    pub async fn test_exchange_info() {
        let client = Mexc::new(None,None,None).unwrap();
        let info = client.exchange_info().await.unwrap();
        dbg!(info);
    }


    #[tokio::test]
    pub async fn test_get_spot_orderbook() {
        let client = Mexc::new(None,None,None).unwrap();
        let info = client.get_spot_orderbook("PLSUSDT", Some(5)).await.unwrap();
        dbg!(info);
    }

    #[tokio::test]
    pub async fn test_send_order() {
        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let receipe = client.new_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009512, 599971.13, None).await.unwrap();
        dbg!(receipe);
    }

    #[test]
    pub fn test_decode_order_receipe() {
        let or = r#"{"symbol":"PLSUSDT","orderId":"C02__426060921085927424065","orderListId":-1,"price":"0.00009512","origQty":"599971.13","type":"MARKET","side":"BUY","transactTime":1717363075282}"#;

        let receipe: OrderReceipe = serde_json::from_str(or).unwrap();
        dbg!(receipe);
    }

}