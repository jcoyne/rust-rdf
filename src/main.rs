#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate curl;
use curl::http;
use std::str;


fn main() {
  let resp = http::handle()
    .get("http://localhost:8983/fedora/rest")
    .exec().unwrap();

  let body_string = match str::from_utf8(resp.get_body()) {
      Some(e) => e,
      None => panic!("Invalid UTF-8 sequence"),
  };

  turtle::parse(body_string);

  println!("code={}; headers={}; body={}",
     resp.get_code(), resp.get_headers(), body_string);
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
    let lines: Vec<&str> = raw.split('\n').collect();


    for line in lines.iter() {
      let matched = regex!(r"\A<[^>]+>").is_match(*line);
      if matched {
        println!("Found a line that has data:\n  {}", line);
      }
    }
    rdf::URIorLiteral::URI(rdf::URI("http://baz".to_string()));

    g.statements = vec![rdf::Statement { subject: rdf::URI("http://foo".to_string()), predicate: rdf::URI("http://bar".to_string()), object: rdf::URIorLiteral::URI(rdf::URI("http://baz".to_string())) }];
    return g;
  }
}
