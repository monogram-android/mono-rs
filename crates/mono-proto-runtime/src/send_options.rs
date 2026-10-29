use crate::error::MtprotoError;
pub const SCHEDULE_WHEN_ONLINE: i32 = 0x7fff_fffe;

use mono_proto::latest::api::*;

#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    pub silent: bool,
    pub no_webpage: bool,
    pub schedule_date: Option<i32>,
}

impl SendOptions {
    pub fn validate(&self, random_id: i64) -> Result<(), MtprotoError> {
        if let Some(date) = self.schedule_date {
            validate_date(date)?;
            if random_id == 0 {
                return Err(MtprotoError::Message(
                    "scheduled sends require a persisted random id".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn apply_text(&self, request: &mut MessagesSendMessageRequest) {
        if self.no_webpage {
            request.flags |= MessagesSendMessageRequest::NO_WEBPAGE_FLAG;
        }
        if self.silent {
            request.flags |= MessagesSendMessageRequest::SILENT_FLAG;
        }
        if self.schedule_date.is_some() {
            request.flags |= MessagesSendMessageRequest::SCHEDULE_DATE_FLAG;
        }
        request.no_webpage = self
            .no_webpage
            .then(|| Box::new(True::True(TrueConstructor {})));
        request.silent = self
            .silent
            .then(|| Box::new(True::True(TrueConstructor {})));
        request.schedule_date = self.schedule_date;
    }

    pub fn apply_media(&self, request: &mut MessagesSendMediaRequest) {
        if self.silent {
            request.flags |= MessagesSendMediaRequest::SILENT_FLAG;
        }
        if self.schedule_date.is_some() {
            request.flags |= MessagesSendMediaRequest::SCHEDULE_DATE_FLAG;
        }
        request.silent = self
            .silent
            .then(|| Box::new(True::True(TrueConstructor {})));
        request.schedule_date = self.schedule_date;
    }

    pub fn apply_album(&self, request: &mut MessagesSendMultiMediaRequest) {
        if self.silent {
            request.flags |= MessagesSendMultiMediaRequest::SILENT_FLAG;
        }
        if self.schedule_date.is_some() {
            request.flags |= MessagesSendMultiMediaRequest::SCHEDULE_DATE_FLAG;
        }
        request.silent = self
            .silent
            .then(|| Box::new(True::True(TrueConstructor {})));
        request.schedule_date = self.schedule_date;
    }
}

pub fn validate_date(date: i32) -> Result<(), MtprotoError> {
    if date <= 0 || date == i32::MAX {
        return Err(MtprotoError::Message("invalid scheduled date".into()));
    }
    Ok(())
}
