use std::collections::HashMap;
use crate::base::service::BaseService;
use std::any::{TypeId, Any};
use crate::base::cache::BaseCache;
use crate::base::error::CustomError;
use std::rc::Rc;
use std::cell::Cell;
