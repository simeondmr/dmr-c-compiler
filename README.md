# dmr C Compiler
It's a small C compiler for x86-64 architecture

## 🚀 Getting Started
The project is still a work in progress, but you can try to compile the examples in the [`/examples`](./examples)

### 🔧 Usage
To generate the assembly of an example file, run the compiler with the following command:

```sh
./dmr_c_compiler /examples/binary_operators/and_or_xor.c
```

## Note

Currently only int variable, expression and return statement are supported.
As you can see from the generated assembly, the code actually is not optimized but in the future I will implement this pass.

## Important
Please open an issue if you find a bug.\
Feel free to fork the project if you wanna join.

dmrsim-117