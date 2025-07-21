use std::collections::VecDeque;

use crate::models::KlineEvent;

#[derive(Default, Clone, Debug)]
pub struct SimpleStrategy {
    current_kline: Option<KlineEvent>,
    previous_kline: Option<KlineEvent>,
}

impl SimpleStrategy {
    // Allows stored prices to be stored.
    pub fn update(&mut self, kline: &KlineEvent) {
        self.previous_kline = self.current_kline.clone();
        self.current_kline = Some(kline.to_owned());
    }
}

impl super::decision::TradingDecision for SimpleStrategy {
    fn evaluate_favourable_direction(&self) -> Option<super::decision::PositionDirection> {
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

#[derive(Default, Clone, Debug)]
pub struct SimpleAverage {
    prices: VecDeque<KlineEvent>,
}

impl SimpleAverage {
    pub fn update(&mut self, kline: &KlineEvent) {
        if self.prices.len() == 10 {
            self.prices.pop_front();
        }
        self.prices.push_back(kline.to_owned());
    }
}

impl super::decision::TradingDecision for SimpleAverage {
    fn evaluate_favourable_direction(&self) -> Option<super::decision::PositionDirection> {
        let mean = self.prices.iter().map(|a| a.k.c).sum::<f64>() / self.prices.len() as f64;
        if let Some(value) = self.prices.iter().last() {
            if value.k.c > mean {
                return Some(super::decision::PositionDirection::Long);
            }
        }
        None
    }
}
