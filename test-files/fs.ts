const text = `Hello, World!`;
const encoder = new TextEncoder();
const data = encoder.encode(text);

await Deno.writeFile("hello.txt", data);

console.log("File written successfully!");
