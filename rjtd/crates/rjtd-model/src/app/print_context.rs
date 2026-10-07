use crate::{DocumentCore, Error, Result, valid_print_date};

impl DocumentCore {
    /// Render context only; cached field values and raw document data stay intact.
    pub fn set_print_date(&mut self, value: &str) -> Result<()> {
        if !valid_print_date(value) {
            return Err(Error::InvalidData(
                "print date must be a valid YYYY/MM/DD date".into(),
            ));
        }
        self.print_date = Some(value.to_string());
        Ok(())
    }
}
