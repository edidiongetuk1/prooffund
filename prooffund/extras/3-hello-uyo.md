# 3. Hello Uyo

You'll put a small contract on the network and call it. Open a terminal in the `build-day-uyo` folder.

## Set up your account

These three lines pick testnet, create an account called alice, and make alice your default.

```bash
stellar network use testnet
```

```bash
stellar keys generate alice --fund
```

```bash
stellar keys use alice
```

## Read the contract

Open [contracts/hello-uyo/src/lib.rs](../contracts/hello-uyo/src/lib.rs). You don't need to know Rust to follow it.

```rust
#[contractimpl]
impl HelloUyo {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}
```

Everything inside `#[contractimpl]` can be called from outside. This function takes a name and returns two words.

## Build it

```bash
stellar contract build
```

The first build is slow, so use the time to meet your neighbour. It's done when you see `Build Complete`.

## Put it on the network

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/hello_uyo.wasm --alias hello
```

You'll get back an ID starting with C. That's your contract's address. The alias means you can call it `hello` instead.

Can't build? Use the ready-made one:

```bash
stellar contract deploy --wasm wasm/hello_uyo.wasm --alias hello
```

## Call it

```bash
stellar contract invoke --id hello -- hello --to Uyo
```

You should see `["Hello","Uyo"]`. Your code is now running on a public network.

Put your green note up.

## Make it yours

Change `"Hello"` in `lib.rs` to your own greeting. Use Pidgin, Ibibio, Efik or anything else. Then build, deploy and call it again:

```bash
stellar contract build
```

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/hello_uyo.wasm --alias hello
```

```bash
stellar contract invoke --id hello -- hello --to Uyo
```

Your new contract gets a new address, because a deployed contract doesn't change by itself. The CLI moves the alias to the new one.

## Sign the wall from your terminal

You signed the wall from your phone. Now call the same contract from here. First save its address:

```bash
stellar contract alias add wall --id CB5EZ2C3JXSXW7PNCQ6EE7GLKUKL332REK62ARDBH3UUNNY2LQAD2R5I
```

Then sign it, using your own name and message:

```bash
stellar contract invoke --id wall -- sign --who alice --name "Your name" --message "Hello from my terminal"
```

Look at the big screen. Your phone and your terminal both talked to the same contract, and the contract doesn't care which one you used.
