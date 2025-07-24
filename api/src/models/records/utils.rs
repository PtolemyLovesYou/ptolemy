use ptolemy::error::ParseError;
use ptolemy::models::{Id, Tier};
use uuid::Uuid;

#[allow(clippy::type_complexity)] // this is literally the easiest way to do it
pub fn get_foreign_keys(
    parent_id: Id,
    tier: &Tier,
) -> Result<(Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>), ParseError> {
    match tier {
        Tier::System => Ok((Some(parent_id.into()), None, None, None)),
        Tier::Subsystem => Ok((None, Some(parent_id.into()), None, None)),
        Tier::Component => Ok((None, None, Some(parent_id.into()), None)),
        Tier::Subcomponent => Ok((None, None, None, Some(parent_id.into()))),
    }
}
