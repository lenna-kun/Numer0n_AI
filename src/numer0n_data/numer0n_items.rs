use super::numer0n_item;

pub struct Numer0nItems(pub Vec<numer0n_item::Numer0nItem>);
impl std::fmt::Display for Numer0nItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut comma_separated = String::new();

        for numer0n_item in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&*format!("{}", numer0n_item));
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&*format!("{}", &self.0[self.0.len() - 1]));
        write!(f, "[{}]", comma_separated)
    }
}