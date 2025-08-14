use rust2go::RegenArgs;

fn main() {
    rust2go::Builder::new()
        .with_go_src("../../../libos-entry/entry/cvm")
        .with_regen_arg(RegenArgs {
            src: "./src/tee_server.rs".into(),
            dst: "../../../libos-entry/entry/cvm/gen.go".into(),
            go118: true,
            ..Default::default()
        })
        .build();
}
