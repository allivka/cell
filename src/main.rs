use cell::Cell;

fn main() -> std::io::Result<()> {
    let app = Cell::default();
    app.run()
}
