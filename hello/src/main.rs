use hello::greet; // like an import

fn main() {
    // hello::greet(); // absolute path calling
    greet(); // can now use because we've imported (use) 
}
