impl super::Field {
    pub fn to_string(&self) -> String {
        format!(
            r#"   1  2  3  4  5  6  7  8  9
A  {}
B  {} 
C  {}
D  {}
E  {}
F  {}
G  {}
H  {}
I  {}"#,
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i
        )
    }
}

impl std::fmt::Display for super::Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
