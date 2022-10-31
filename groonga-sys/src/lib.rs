#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_void, CString};
    use std::mem::zeroed;
    use std::ptr::{null, null_mut};

    #[test]
    fn bindings() {
        let ret = unsafe { grn_init() };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ctx = unsafe { grn_ctx_open(0) };
        assert_eq!(false, ctx.is_null());

        let cmd_ver = unsafe { grn_ctx_get_command_version(ctx) };
        assert_eq!(grn_command_version::GRN_COMMAND_VERSION_1, cmd_ver);

        let db = unsafe { grn_db_create(ctx, null(), zeroed()) };
        assert_eq!(false, db.is_null());

        let key_type = unsafe { grn_ctx_at(ctx, grn_builtin_type::GRN_DB_SHORT_TEXT as u32) };
        assert_eq!(false, key_type.is_null());

        let value_type = unsafe { grn_ctx_at(ctx, grn_builtin_type::GRN_DB_TEXT as u32) };
        assert_eq!(false, value_type.is_null());

        let table = unsafe {
            grn_table_create(
                ctx,
                null(),
                0,
                null(),
                GRN_OBJ_TABLE_HASH_KEY,
                key_type,
                null_mut(),
            )
        };
        assert_eq!(false, table.is_null());

        let cname = CString::new("value").unwrap();
        let column = unsafe {
            grn_column_create(
                ctx,
                table,
                cname.as_ptr(),
                cname.as_bytes().len() as u32,
                null(),
                GRN_OBJ_COLUMN_SCALAR,
                value_type,
            )
        };
        assert_eq!(false, column.is_null());

        let key = CString::new("key").unwrap();
        let id = unsafe {
            grn_table_add(
                ctx,
                table,
                key.as_ptr() as *mut c_void,
                key.as_bytes().len() as u32,
                null_mut(),
            )
        };
        assert_eq!(1, unsafe { grn_table_size(ctx, table) });

        let value = CString::new("hello, world!").unwrap();
        let mut value_obj = unsafe { zeroed::<grn_obj>() };
        unsafe { grn_text_init(&mut value_obj, 0) };
        unsafe { grn_text_put(ctx, &mut value_obj, value.as_ptr(), value.as_bytes().len()) };
        let ret = unsafe { grn_obj_set_value(ctx, column, id, &mut value_obj, GRN_OBJ_SET as i32) };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ret = unsafe { grn_ctx_close(ctx) };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ret = unsafe { grn_fin() };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);
    }
}
