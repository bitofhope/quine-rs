fn main() {
    let a = "fn main() {\n    let a = \"";
    let b = "println!(\n        \"{0}{1}\\\";\\n    let b = \\\"{2}\\\";\\n    {3}\",\n        a,\n        a.escape_default(),\n        b.escape_default(),\n        b\n    );\n}";
    println!(
        "{0}{1}\";\n    let b = \"{2}\";\n    {3}",
        a,
        a.escape_default(),
        b.escape_default(),
        b
    );
}
