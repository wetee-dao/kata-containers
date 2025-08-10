use rust2go::RegenArgs;

fn main() {
    rust2go::Builder::new()
        .with_go_src("../../../tee-server/")
        .with_regen_arg(RegenArgs {
            src: "./src/tee_server.rs".into(),
            dst: "../../../tee-server/gen.go".into(),
            go118: true,
            ..Default::default()
        })
        .build();
}
