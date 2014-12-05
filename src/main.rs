#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate curl;
use curl::http;
use std::str;

use std::io::File;

fn main() {
  //let turtle_file = download("http://localhost:8983/fedora/rest");
  let turtle_file = from_file("sample.rdf.ttl");

  turtle::parse(turtle_file.as_slice());
}

fn from_file(path: &str) -> String {
    let p = Path::new(path);

    match File::open(&p).read_to_string() {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    }
}

fn download(uri: &str) -> String {
  let resp = http::handle()
    .get(uri)
    .exec().unwrap();

  if resp.get_code() != 200 {
    panic!("Server didn't give 200 response");
  };

  match str::from_utf8(resp.get_body()) {
      Some(e) => e.to_string(),
      None => panic!("Invalid UTF-8 sequence"),
  }
}

mod rdf {
  pub struct URI(pub String);

  pub enum URIorLiteral {
    URI(URI),
    Literal(String)
  }

  pub struct Statement {
      pub subject: URI,
      pub predicate: URI,
      pub object: URIorLiteral,
  }

  pub struct Graph {
      pub statements: Vec<Statement>
  }

  impl Graph {
      pub fn new() -> Graph {
          Graph { statements: vec![] }
      }
  }
}


mod turtle {
  use rdf;

  pub fn parse(raw: &str) -> rdf::Graph {
    let mut g = rdf::Graph::new();
    let statements_re = regex!(r"\.\n+");
    let statements: Vec<&str> = statements_re.split(raw).collect();

    let prefix_re = regex!(r"\A@prefix (\S*): <([^>]*)>");
    let triples_re = regex!(r"(?ms)\A<([^>]*)> (.*)");

    for line in statements.iter() {
        let prefix_result = prefix_re.captures(*line);
        if prefix_result.is_some() {
            let unwrapped = prefix_result.unwrap();
            println!("Found a prefix:\n  {} -> {}", unwrapped.at(1), unwrapped.at(2));
        } else {
            let triples_result = triples_re.captures(*line);
            if triples_result.is_some() {
                let unwrapped = triples_result.unwrap();
                println!("Found a mess of triples:\n  Subject: {} -> mess: {}", unwrapped.at(1), unwrapped.at(2));
            }
        }
    }
    rdf::URIorLiteral::URI(rdf::URI("http://baz".to_string()));

    g.statements = vec![rdf::Statement { subject: rdf::URI("http://foo".to_string()), predicate: rdf::URI("http://bar".to_string()), object: rdf::URIorLiteral::URI(rdf::URI("http://baz".to_string())) }];
    return g;
  }
}
