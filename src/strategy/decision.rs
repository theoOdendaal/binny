use crate::errors;

/// Store current position parameters.
#[derive(Debug, Default)]
pub struct PositionParameters {
    position_direction: Option<PositionDirection>,
    position_action: Option<PositionAction>,
}

impl PositionParameters {
    pub fn direction(&self) -> Option<PositionDirection> {
        self.position_direction.to_owned()
    }

    pub fn action(&self) -> Option<PositionAction> {
        self.position_action
    }

    pub fn set_direction(&mut self, direction: Option<PositionDirection>) {
        self.position_direction = direction;
    }

    pub fn set_action(&mut self, action: Option<PositionAction>) {
        self.position_action = action;
    }
}

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

/// Specifies how a stream event is handled by
/// a trading strategy. Is most circumstances this
/// translates to storing or ingesting the event,
/// rather than application of trading logic.
pub trait HandleStreamEvent<A> {
    fn handle_stream_event(&mut self, event: A) -> Result<(), errors::Error>;
}

/// Trait used to designed a trading strategy.
pub trait TradingStrategy {
    /// Determines favourable position direction.
    fn signal(&self) -> Option<PositionDirection>;

    // Determines what action to take based on the current 'PositionDirection' and
    // favourable position direction.
    fn determine_action(&self, position: Option<PositionDirection>) -> Option<PositionAction> {
        let signal = self.signal();

        // No action required if the current postion is
        // equal to the favourable direction.
        if position == signal {
            return None;
        }

        // TODO: potentially develop more complex conditions that allow unfavourable to be offset
        // using other positions rather than just selling? Which would require this condition to
        // be revisted, as the current position not longer have to be 'None'.

        // Only 'Buy' when current position is 'None'.
        if let Some(direction) = signal {
            if position == None {
                return Some(PositionAction::Buy(direction));
            }
        }

        Some(PositionAction::Sell)
    }

    // TODO: Refactor this function to make it more understandable.

    /// Update a 'PositionDirection' using a 'PositionAction'.
    fn determine_direction(
        &self,
        position: Option<PositionDirection>,
        action: Option<PositionAction>,
    ) -> Option<PositionDirection> {
        match (position, action) {
            (_, None) => position,
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
}
