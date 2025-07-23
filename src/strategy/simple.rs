use std::collections::VecDeque;

use crate::models::KlineEvent;

#[derive(Default, Clone, Debug)]
pub struct SimpleStrategy {
    current_kline: Option<KlineEvent>,
    previous_kline: Option<KlineEvent>,
}

impl super::decision::TradingStrategy for SimpleStrategy {
    fn signal(&self) -> Option<super::decision::PositionDirection> {
        if let (Some(current), Some(previous)) =
            (self.current_kline.clone(), self.previous_kline.clone())
        {
            if current.k.c >= previous.k.c {
                return Some(super::decision::PositionDirection::Long);
            }
        }
        Some(super::decision::PositionDirection::Short)
    }
}

impl super::decision::HandleStreamEvent<&KlineEvent> for SimpleStrategy {
    fn handle_stream_event(&mut self, event: &KlineEvent) -> Result<(), crate::errors::Error> {
        self.previous_kline = self.current_kline.clone();
        self.current_kline = Some(event.to_owned());
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct SimpleAverage {
    prices: VecDeque<KlineEvent>,
}

impl super::decision::TradingStrategy for SimpleAverage {
    fn signal(&self) -> Option<super::decision::PositionDirection> {
        let mean = self.prices.iter().map(|a| a.k.c).sum::<f64>() / self.prices.len() as f64;
        if let Some(value) = self.prices.iter().last() {
            if value.k.c > mean {
                return Some(super::decision::PositionDirection::Long);
            }
        }
        None
    }
}

impl super::decision::HandleStreamEvent<&KlineEvent> for SimpleAverage {
    fn handle_stream_event(&mut self, event: &KlineEvent) -> Result<(), crate::errors::Error> {
        if self.prices.len() == 30 {
            self.prices.pop_front();
        }
        self.prices.push_back(event.to_owned());
        Ok(())
    }
}
