use crate::errors::Error;
use crate::types::MAX_DESCRIPTION_LENGTH;
use crate::types::MAX_ITEM_NAME_LENGTH;

pub fn validate_item_name(name: &str, description: Option<&str>) -> Result<(), Error> {
    if name.is_empty() {
        return Err(Error::Custom(
            "How the hell did you even pass this empty??".to_string(),
        ));
    }
    if name.chars().count() > MAX_ITEM_NAME_LENGTH {
        return Err(Error::Custom(
            "item name too long (max 256 characters).".to_string(),
        ));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || " -_'.,!?()".contains(c))
    {
        return Err(Error::Custom(
            "item names can only contain english letters, numbers, and basic punctuation."
                .to_string(),
        ));
    }
    let Some(description) = description else {
        return Ok(());
    };
    if description.chars().count() > MAX_DESCRIPTION_LENGTH {
        return Err(Error::Custom(
            "description too long (max 4096 characters).".to_string(),
        ));
    }
    if !description
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || " -_'.,!?()".contains(c))
    {
        return Err(Error::Custom(
            "description can only contain english letters, numbers, and basic punctuation."
                .to_string(),
        ));
    }
    Ok(())
}
