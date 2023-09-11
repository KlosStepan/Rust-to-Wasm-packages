# Pwn Rust - Hello Wasm
This is Rust to Wasm `npm package` providing `greetings` functionality.

These are two functions accessible from our package:
- `greetStatic` - static greeting,
- `greet` - function calling back JavaScript's `alert` function back from Rust.  

Rust code `src/lib.rs` has function definitions and binds.
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn greetStatic() -> String {
    "Hello from Rust!".to_string()
}
```

These functions from package can be used in React code as follows.  
TypeScript type
```ts
type IRustModule = {
  greet: (message: string) => null;
  greetStatic: () => string;
};
```
An actual import and usage in your (our [^1]) application
```ts
export default function Home() {
    const [rustModule, setRustModule] = useState<IRustModule | null>(null);
    useEffect(() => {
        // Dynamically import the Rust Wasm module
        import('pwnrusthellowasm').then((module: any) => {
        setRustModule(module);
    });
  }, []);
  const handleGreetClick = () => {
    if (rustModule) {
        const { greet } = rustModule;
        const result = greet('Calling Rust function greet(&str), popping back JS alert from Rust.');
    }
  };

  const handleGreetStaticClick = () => {
    if (rustModule) {
        const { greetStatic } = rustModule;
        const result = greetStatic();
        console.log(result);
    }
  };

  return (
    <>
        <span>Hello Wasm demonstration</span>
        <button onClick={handleGreetClick}>Greet</button>
        <button onClick={handleGreetStaticClick}>Greet Static</button>
        <div>
            Whatever content...
        </div>
    </>
  )
}
```

[^1]: https://github.com/KlosStepan/Next-ft-Wasm