const { ops } = globalThis.Deno.core;

function exampleCustomOp(str) {
  console.log("ops", Object.keys(ops));
  return ops.custom_op_example(str);
}

globalThis.ExampleExtension = { exampleCustomOp };
