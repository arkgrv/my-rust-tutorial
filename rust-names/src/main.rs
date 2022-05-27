mod data_interface;

fn main() {
    const URL: &str = "https://randomuser.me/api";
    let data = data_interface::get_data(URL);
    let json_data = data_interface::data_to_json(&data);
    let data = json_data.unwrap();

    println!("{} {}", data["results"][0]["name"]["first"], data["results"][0]["name"]["last"]);
}
