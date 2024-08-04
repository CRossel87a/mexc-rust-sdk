

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::orders::CancelledOrder;
    use crate::orders::Order;
    use crate::orders::OrderSide;
    use crate::orders::OrderType;
    use crate::orders::OrderReceipt;
    use crate::utils::round;
    use crate::{utils::unlock_keys, Mexc};

    async fn sleep(secs: f64) {
        tokio::time::sleep(Duration::from_secs_f64(secs)).await;
    }


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

        let receipe = client.submit_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009512, 599971.13, None).await.unwrap();
        dbg!(receipe);
    }

    #[test]
    pub fn test_decode_order_receipe() {
        let or = r#"{"symbol":"PLSUSDT","orderId":"C02__426060921085927424065","orderListId":-1,"price":"0.00009512","origQty":"599971.13","type":"MARKET","side":"BUY","transactTime":1717363075282}"#;

        let receipe: OrderReceipt = serde_json::from_str(or).unwrap();
        dbg!(receipe);
    }

    #[test]
    pub fn test_decode_cancelled_orders() {
        let co = r#"[{"symbol":"PLSUSDT","orderId":"C02__426199983784497153065","price":"0.00009712","origQty":"299985.56","type":"LIMIT","side":"SELL","executedQty":"0","cummulativeQuoteQty":"0","status":"NEW"},{"symbol":"PLSUSDT","orderId":"C02__426199982572318720065","price":"0.00009512","origQty":"299985.56","type":"LIMIT","side":"SELL","executedQty":"0","cummulativeQuoteQty":"0","status":"NEW"}]"#;
        let cancelled_orders: Vec<CancelledOrder> = serde_json::from_str(co).unwrap();
        dbg!(cancelled_orders);
    }

    #[tokio::test]
    pub async fn test_send_cancel_all_orders() {
        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let q = 599971.13;

        let order_1 = client.submit_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009512, round(q/2.0, 2), None).await.unwrap();
        dbg!(order_1);

        let order_2 = client.submit_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009712, round(q/2.0, 2), None).await.unwrap();
        dbg!(order_2);

        sleep(0.4).await;

        let co = client.cancel_all_orders("PLSUSDT", None).await.unwrap();
        dbg!(co);
    }

    #[tokio::test]
    pub async fn test_send_cancel_order() {
        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let q = 599971.13;

        let order_1 = client.submit_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009512, round(q/2.0, 2), None).await.unwrap();
        dbg!(&order_1);

        sleep(0.4).await;

        let co = client.cancel_order("PLSUSDT", &order_1.order_id, None).await.unwrap();
        dbg!(co);
    }

    #[tokio::test]
    pub async fn test_batch_order() {

        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let mut orders = vec![];

        let q = 599971.13;
        let q = round(q/2.0, 2);

        orders.push(Order {
            symbol: "PLSUSDT".into(),
            price: 0.00009512,
            quantity: q,
            side: OrderSide::SELL,
            order_type: OrderType::LIMIT
        });
        
        orders.push(Order {
            symbol: "PLSUSDT".into(),
            price: 0.00009712,
            quantity: q,
            side: OrderSide::SELL,
            order_type: OrderType::LIMIT
        });

        let res = client.batch_orders(orders, None).await.unwrap();
        dbg!(res);
    }

    #[tokio::test]
    pub async fn test_open_orders() {
        let (key, secret) = unlock_keys().unwrap();
        let client = Mexc::new(Some(key),Some(secret),None).unwrap();

        let q = 599971.13;

        let order_1 = client.submit_order("PLSUSDT", OrderSide::SELL, OrderType::LIMIT, 0.00009512, round(q/2.0, 2), None).await.unwrap();
        dbg!(&order_1);

        sleep(0.4).await;

        let orders = client.get_open_orders("PLSUSDT", None).await.unwrap();
        dbg!(orders);
    }
}