

pub const SUGGESTIONS: [&str; 6] = [ 
        "if let Some(result) = Some(\"hello\") { println!(\"{}\", result); }",
    "let set = HashSet::new();",
    "fn process<T>(input: i32) -> Option<T> { unimplemented!() }",
    "let buf = [0u8; 1024]; stream.read(&mut buf)?;",
    "let reversed = String::from(\"Ferris\").chars().rev().collect::<String>();",
    "let result = vec.iter().filter(|x| x.is_some()).collect::<Vec<_>>();",
];