# Deno Runtime Rust

Proof of concept of creating a JavaScript runtime with Rust by using the Deno Runtime crate. Useful for embedding and customizing Deno's runtime in your Rust application or creating a custom JavaScript runtime based on Deno's runtime. This repo showcases:
- Importing from https sources like esm.sh and handling TypeScript files
- Handling permissions with custom prompter
- Writing extensions

I'll continue to add more features and examples as I need them for my projects. I hope this helps you too!

## Useful guides

These are useful guides that helped me undersand Deno's internals andhow to use the crates better:
- Deno's "Roll your own JavaScript runtime" series:
    - Part 1: https://deno.com/blog/roll-your-own-javascript-runtime
    - Part 2: https://deno.com/blog/roll-your-own-javascript-runtime-pt2
    - Part 3: https://deno.com/blog/roll-your-own-javascript-runtime-pt3
- Deno 2 internals by [Divy Srivastava](https://github.com/littledivy): https://littledivy.com/deno-2.html
