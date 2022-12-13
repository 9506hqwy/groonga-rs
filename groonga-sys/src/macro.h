void grn_record_init(grn_obj *obj, uint8_t flgas, grn_id domain);

void grn_text_init(grn_obj *obj, uint8_t flags);

void grn_text_put(grn_ctx *ctx, grn_obj *obj, const char *str, size_t len);
