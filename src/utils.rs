use ring::digest;

pub fn generate_sha256(number: u32) -> String {
    let number_bytes = number.to_string();
    let mut context = digest::Context::new(&digest::SHA256);
    
    context.update(number_bytes.as_bytes());

    hex::encode(context.finish())
}
