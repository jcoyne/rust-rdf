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
    #[deriving(Clone, Show)]
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
  // https://gist.github.com/danlentz/6564037#file-turtle-bnf-md
  use rdf;

  struct PredicateObjectEntry {
      predicate: rdf::URI,
      object_list: String,
  }

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
                let subject = unwrapped.at(1);
                // TODO this is causing a panic
                let predicate_object_tuples = parse_predicate_object_list(unwrapped.at(2));
                for s in predicate_object_tuples.iter() {
                    g.statements.push(
                        // TODO put object list members as the object
                        rdf::Statement { subject: rdf::URI(subject.to_string()), predicate: s.predicate.clone(), object: rdf::URIorLiteral::URI(rdf::URI("http://baz".to_string())) }
                    )
                }
            }
        }
    }

    for s in g.statements.iter() {
        println!("Statement {} {} ", s.subject, s.predicate);
    }

    return g;
  }

  fn parse_predicate_object_list(raw: &str) -> Vec<PredicateObjectEntry> {
      //let mut split = raw.to_string().split(" ;");
      let v: Vec<&str> = raw.split_str(" ;").collect();
      let re = regex!(r"\A([^\s]*) (.*)$");

      let mut pred_object_list: Vec<PredicateObjectEntry> = Vec::new();
      for s in v.iter() {
          let predicate_result = re.captures(*s);
          if predicate_result.is_some() {
              let unwrapped = predicate_result.unwrap();
              // TODO split object_list by commas.
              let entry = PredicateObjectEntry { predicate: rdf::URI(unwrapped.at(1).to_string()), object_list: unwrapped.at(2).to_string() };
              pred_object_list.push(entry);
          }
      }
      return pred_object_list;
  }
}
