#include <groonga/groonga.h>

void grn_record_init(grn_obj *obj, uint8_t flags, grn_id domain) {
  GRN_RECORD_INIT(obj, flags, domain);
}

void grn_text_init(grn_obj *obj, uint8_t flags) {
  GRN_TEXT_INIT(obj, flags);
}

void grn_text_put(grn_ctx *ctx, grn_obj *obj, const char *str, size_t len) {
  GRN_TEXT_PUT(ctx, obj, str, len);
}
