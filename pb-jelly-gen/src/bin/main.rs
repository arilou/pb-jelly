use pb_jelly_gen::gen_protos;

fn main() -> std::io::Result<()> {
    gen_protos(vec!["./protos"]);

    Ok(())
}
