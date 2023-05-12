use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn formate(&self, data: &Data, buf: &mut String);
}

struct Report;

impl Report {
    fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        g.formate(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn formate(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {} \n", k, v);
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn formate(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}:"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop();
        }
        buf.push(']');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strategy() {
        let mut a = String::from("");
        Report::generate(Text, &mut a);
        println!("{}", a);
        a.clear();
        Report::generate(Json, &mut a);
        println!("{}", a);
    }
}
