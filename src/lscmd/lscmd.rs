use curl::easy::Easy2;
use serde::Deserialize;
use std::io::{stdout, Write};
use tokio::sync::oneshot::Sender;

#[derive(Deserialize, Debug)]
pub struct Service {
    pub id: i64,
    pub name: String,
    pub r#type: i64,
}
use curl::easy::{Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn lscmd(url: &str, tx: Sender<Vec<Service>>) -> std::result::Result<(), curl::Error> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true)?;
    easy.url(url)?;
    easy.perform()?;

    let data = &easy.get_ref().0;
    let s = &*String::from_utf8_lossy(data);
    let a: serde_json::error::Result<Vec<Service>> = serde_json::from_str(s);
    match a {
        Ok(a) => {
            let result: Vec<_> = a.into_iter().filter(|v| v.r#type == 192).collect();
            tx.send(result).unwrap();
        }
        Err(e) => {
            stdout().write_all(data).unwrap();
            println!();
            panic!("{}", e);
        }
    }
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
