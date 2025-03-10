use biscuit_auth::{builder::BlockBuilder, datalog::SymbolTable, Biscuit, KeyPair};
use rand::{prelude::StdRng, SeedableRng};

fn main() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let root = KeyPair::new_with_rng(&mut rng);
    let external = KeyPair::new_with_rng(&mut rng);

    let mut builder = Biscuit::builder();

    let external_pub = hex::encode(external.public().to_bytes());

    builder
        .add_check(
            format!("check if external_fact(\"hello\") trusting ed25519/{external_pub}").as_str(),
        )
        .unwrap();

    let biscuit1 = builder
        .build_with_rng(&root, SymbolTable::default(), &mut rng)
        .unwrap();

    println!("biscuit1: {}", biscuit1.print());

    let serialized_req = biscuit1.third_party_request().unwrap().serialize().unwrap();

    let req = biscuit_auth::ThirdPartyRequest::deserialize(&serialized_req).unwrap();
    let mut builder = BlockBuilder::new();
    builder.add_fact("external_fact(\"hello\")").unwrap();
    let res = req.create_block(&external.private(), builder).unwrap();

    let biscuit2 = biscuit1.append_third_party(external.public(), res).unwrap();

    println!("biscuit2: {}", biscuit2.print());

    let mut authorizer = biscuit1.authorizer().unwrap();
    authorizer.allow().unwrap();
    println!("authorize biscuit1:\n{:?}", authorizer.authorize());
    println!("world:\n{}", authorizer.print_world());

    let mut authorizer = biscuit2.authorizer().unwrap();
    authorizer.allow().unwrap();
    println!("authorize biscuit2:\n{:?}", authorizer.authorize());
    println!("world:\n{}", authorizer.print_world());
}
