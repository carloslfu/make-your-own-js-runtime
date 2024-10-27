import { op_hello } from "ext:core/ops";
function hello(str) {
  op_hello(str);
}

globalThis.Extension = { hello };
