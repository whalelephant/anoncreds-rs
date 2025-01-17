use ffi_support::FfiStr;

use super::error::{catch_error, ErrorCode};
use super::object::ObjectHandle;
use super::util::FfiStrList;
use crate::services::{issuer::create_schema, types::Schema};

#[no_mangle]
pub extern "C" fn anoncreds_create_schema(
    schema_name: FfiStr,
    schema_version: FfiStr,
    attr_names: FfiStrList,
    seq_no: i64,
    result_p: *mut ObjectHandle,
) -> ErrorCode {
    catch_error(|| {
        check_useful_c_ptr!(result_p);
        let schema_name = schema_name
            .as_opt_str()
            .ok_or_else(|| err_msg!("Missing schema name"))?;
        let schema_version = schema_version
            .as_opt_str()
            .ok_or_else(|| err_msg!("Missing schema version"))?;
        let schema = create_schema(
            schema_name,
            schema_version,
            attr_names.to_string_vec()?.into(),
            if seq_no > 0 {
                Some(seq_no as u32)
            } else {
                None
            },
        )?;
        let handle = ObjectHandle::create(schema)?;
        unsafe { *result_p = handle };
        Ok(())
    })
}

impl_anoncreds_object!(Schema, "Schema");
impl_anoncreds_object_from_json!(Schema, anoncreds_schema_from_json);
