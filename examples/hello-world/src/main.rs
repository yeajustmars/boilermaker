fn main() {
    println!(
        r"Welcome to Boilermaker!

        This is the `hello-world` example template. It is a minimal Rust project template that 
        demonstrates some Boilermaker basics.

        In the next line of Rust code, we will interpolate a variable from the `boilermaker.toml` 
        file found at the root of the Boilermaker template.

        You can find this variable at:
            `template root` -> `boilermaker.toml` -> `boilermaker.variables.welcome_message`
    "
    );

    println!("{{welcome_message}}");
}
