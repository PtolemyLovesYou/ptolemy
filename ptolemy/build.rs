use apollo_compiler::{hir::OperationType, ApolloCompiler, HirDatabase};
use heck::ToShoutySnakeCase;
use std::fs;
use std::path::PathBuf;

fn gql_compiler() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = ApolloCompiler::new();
    let schema_dir = PathBuf::from("graphql/schema.gql");
    let query_dir = PathBuf::from("graphql/query.gql");
    let mutation_dir = PathBuf::from("graphql/mutation.gql");
    let schema = fs::read_to_string(&schema_dir)?;
    let query = fs::read_to_string(&query_dir)?;
    let mutation = fs::read_to_string(&mutation_dir)?;

    compiler.add_type_system(&schema, "");
    compiler.add_executable(&query, "");
    compiler.add_executable(&mutation, "");

    let diagnostics = compiler.validate();

    for diagnostic in &diagnostics {
        println!("WARNING: {}", diagnostic);
    }

    println!(
        "cargo:info=# Compiling {} operations",
        compiler.db.all_operations().len()
    );

    let mut queries: Vec<String> = Vec::new();
    let mut mutations: Vec<String> = Vec::new();

    for op in &*compiler.db.all_operations() {
        let op = op.clone();
        let name = op.name().expect("All queries must have a name.");
        match op.operation_ty() {
            OperationType::Query => queries.push(name.to_string()),
            OperationType::Mutation => mutations.push(name.to_string()),
            OperationType::Subscription => panic!("Subscriptions are not supported"),
        };
    }

    let out_dir = PathBuf::from("src/generated/gql.rs");

    let mut query_string = "// This file is @generated by ptolemy <3.\n\n".to_string();
    for query in queries {
        query_string.push_str(&format!(
            "pub const {}_QUERY: &'static str = r###\"{}\"###;\n\n",
            query.to_shouty_snake_case(),
            query
        ));
    }

    for mutation in mutations {
        query_string.push_str(&format!(
            "pub const {}_MUTATION: &'static str = r###\"{}\"###;\n\n",
            mutation.to_shouty_snake_case(),
            mutation
        ));
    }

    query_string.push_str(&format!(
        "pub const QUERY: &'static str = r###\"{}\"###;\n\n",
        query
    ));
    query_string.push_str(&format!(
        "pub const MUTATION: &'static str = r###\"{}\"###;\n\n",
        mutation
    ));

    fs::write(out_dir, query_string).unwrap();

    println!("cargo:rerun-if-changed=graphql/");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Validate + compile gql queries/mutations and write to a generated Rust file.
    // Only happens if env var COMPILE_GQL is set to 1
    let compile_gql = std::env::var("COMPILE_GQL")
        .map(|s| s == "1")
        .unwrap_or(false);

    if compile_gql {
        gql_compiler()?;
    }

    // Build protobufs and write to src/generated
    // Only happens if env var BUILD_PROTOBUFS is set to 1
    let build_protobufs = std::env::var("BUILD_PROTOBUFS")
        .map(|s| s == "1")
        .unwrap_or(true);

    if build_protobufs {
        tonic_build::configure()
            .build_server(true)
            .out_dir("src/generated")
            .compile_protos(
                &["proto/observer.proto", "proto/query_engine.proto"],
                &["proto/observer", "proto/query_engine"]
            )?;
    }

    Ok(())
}
