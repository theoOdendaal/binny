#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionDirection {
    Long,
    Short,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionAction {
    Buy(PositionDirection),
    Sell,
}

pub trait TradingDecision {
    fn get_position_action(
        &self,
        current_position: Option<PositionDirection>,
    ) -> Option<PositionAction> {
        let favourable_direction = self.evaluate_favourable_direction();

        // No action required if the current postion is
        // equal to the favourable direction.
        if current_position == favourable_direction {
            return None;
        }

        // TODO: potentially develop more complex conditions that allow unfavourable to be offset
        // using other positions rather than just selling? Which would require this condition to
        // be revisted, as the current position not longer have to be 'None'.

        // Only 'Buy' when current position is 'None'.
        if let Some(direction) = favourable_direction {
            if current_position == None {
                return Some(PositionAction::Buy(direction));
            }
        }

        Some(PositionAction::Sell)
    }

    // TODO: Refactor this function to make it more understandable.

    /// Update a 'PositionDirection' using a 'PositionAction'.
    fn get_position_direction(
        &self,
        current_position: Option<PositionDirection>,
        action: Option<PositionAction>,
    ) -> Option<PositionDirection> {
        match (current_position, action) {
            (_, None) => current_position,
            (None, Some(PositionAction::Buy(direction))) => Some(direction),
            (None, Some(PositionAction::Sell)) => {
                panic!("Exiting a 'None' position should not be possible")
            }
            (Some(PositionDirection::Long), Some(PositionAction::Buy(_))) => {
                panic!("Buying while a position already exists should not be possible.")
            }
            (Some(PositionDirection::Long), Some(PositionAction::Sell)) => None,
            (Some(PositionDirection::Short), Some(PositionAction::Buy(_))) => {
                panic!("Buying while a position already exists should not be possible.")
            }
            (Some(PositionDirection::Short), Some(PositionAction::Sell)) => None,
        }
    }

    /// Determines favourable position direction.
    fn evaluate_favourable_direction(&self) -> Option<PositionDirection>;
}
