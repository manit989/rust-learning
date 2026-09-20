use rust_learn::shop::Item;
use rust_learn::shop::inventory;

fn main() {
    let item = Item::new(String::from("Mechanical Keyboard"), 45.0);

    // 1. Public field read karo
    println!("Item Name: {}", item.name);

    // 2. Getter se private field access karo
    println!("Cost Price: ${}", item.get_cost());

    // 3. Submodule call karo
    let in_stock = inventory::check_stock(101);
    println!("In stock? {}", in_stock);

    // --- EXPERIMENT CHECKS (Inhe uncomment karke compiler errors dekho) ---

    // Check A: Struct field privacy fail hona chahiye
    // println!("{}", item.cost_price);

    // Check B: Private function access fail hona chahiye
    // inventory::database_lookup();
}
