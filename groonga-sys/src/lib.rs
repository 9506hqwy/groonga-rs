#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
        assert!(!ctx.is_null());

        let cmd_ver = unsafe { grn_ctx_get_command_version(ctx) };
        assert_eq!(grn_command_version::GRN_COMMAND_VERSION_1, cmd_ver);

        let db = unsafe { grn_db_create(ctx, null(), zeroed()) };
        assert!(!db.is_null());

        let key_type = unsafe { grn_ctx_at(ctx, grn_builtin_type::GRN_DB_SHORT_TEXT as u32) };
        assert!(!key_type.is_null());

        let value_type = unsafe { grn_ctx_at(ctx, grn_builtin_type::GRN_DB_TEXT as u32) };
        assert!(!value_type.is_null());

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
        assert!(!table.is_null());

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
        assert!(!column.is_null());

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

        let default_column_name = CString::new("value").unwrap();
        let mut default_column = unsafe { zeroed::<grn_obj>() };
        unsafe { grn_text_init(&mut default_column, 0) };
        unsafe {
            grn_text_put(
                ctx,
                &mut default_column,
                default_column_name.as_ptr(),
                default_column_name.as_bytes().len(),
            )
        };

        let query = unsafe { grn_expr_create(ctx, null(), 0) };
        assert!(!query.is_null());

        let var = unsafe { grn_expr_add_var(ctx, query, null(), 0) };
        assert!(!var.is_null());

        unsafe { grn_record_init(var, 0, grn_obj_id(ctx, table)) };

        let keyword = CString::new("hello").unwrap();
        let ret = unsafe {
            grn_expr_parse(
                ctx,
                query,
                keyword.as_ptr(),
                keyword.as_bytes().len() as u32,
                &mut default_column,
                grn_operator::GRN_OP_MATCH,
                grn_operator::GRN_OP_AND,
                GRN_EXPR_SYNTAX_QUERY,
            )
        };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let results =
            unsafe { grn_table_select(ctx, table, query, null_mut(), grn_operator::GRN_OP_OR) };
        assert_eq!(grn_rc::GRN_SUCCESS, unsafe { (*ctx).rc });
        assert_eq!(1, unsafe { grn_table_size(ctx, results) });

        let ret = unsafe { grn_expr_close(ctx, query) };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ret = unsafe { grn_obj_close(ctx, db) };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ret = unsafe { grn_ctx_close(ctx) };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);

        let ret = unsafe { grn_fin() };
        assert_eq!(grn_rc::GRN_SUCCESS, ret);
    }
}
