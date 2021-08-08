use curl::easy::Easy;
use serde::Deserialize;
use std::io::{stdout, Write};

#[derive(Deserialize, Debug)]
pub struct Service {
    id: i64,
    name: String,
    r#type: i64,
}

pub fn lscmd<F: FnMut(Vec<Service>) -> () + Send + 'static>(
    url: &str,
    mut f: F,
) -> std::result::Result<(), curl::Error> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.write_function(move |data| {
        let s = &*String::from_utf8_lossy(data);
        let a: serde_json::error::Result<Vec<Service>> = serde_json::from_str(s);
        match a {
            Ok(a) => {
                let b: Vec<_> = a.into_iter().filter(|v| v.r#type == 192).collect();
                f(b);
                Ok(data.len())
            }
            Err(e) => {
                stdout().write_all(data).unwrap();
                println!();
                panic!("{}", e);
            }
        }
    })?;
    easy.perform()?;
    Ok(())
}

#[test]
fn test_de() {
    let a: Vec<Service> = serde_json::from_str(
        r#"
        [
            {"id":1,"name":"name 1","type":1,"extra_field_1":{}},
            {"id":2,"name":"name 2","type":192,"extra_field_2":[]}
        ]
        "#,
    )
    .unwrap();
    let b: Vec<_> = a.into_iter().filter(|v| v.r#type == 1).collect();
    assert_eq!(b.len(), 1);
}
