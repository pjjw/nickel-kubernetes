use tweag_gen_k8s_nickel::as_ncl::{AsNcl};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let path_to_openapi_def = args.get(1).map_or("error", |x|x);
  match openapi::from_path(path_to_openapi_def) {
    Ok(spec) => {
      for (name, schema) in spec.definitions.into_iter() {
        if name != "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta" {
          continue
        }
        //println!("name: {:?}", name);
        //println!("spec: {:?}", schema);
        let mut a = "{\n".to_string();
        let b = schema.to_ncl(name.as_str(), 2);
        let c = "}\n";
        a.push_str(b.as_str());
        a.push_str("\n");
        a.push_str(c);

        std::fs::write("result.ncl", a);
      };
    },
    Err(err) => println!("error: {}", err)
  }
}