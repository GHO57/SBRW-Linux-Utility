use tauri::{Emitter, Window};

use crate::types::{error::CustomError, wizard::WizardStep};

pub fn report_step(window: &Window, step: WizardStep) -> Result<(), CustomError> {
    window.emit("wizard-step", step)?;
    Ok(())
}

pub fn report_event(window: &Window, event: &str, payload: &str) -> Result<(), CustomError> {
    window.emit(event, payload)?;
    Ok(())
}
