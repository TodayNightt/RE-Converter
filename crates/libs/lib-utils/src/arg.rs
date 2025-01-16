use std::marker::PhantomData;

pub struct Arg<F, V, S> {
    flag: String,
    without_dash: bool,
    double_dash: bool,
    value_double_quote: bool,
    value: Option<String>,
    value_spacer: Option<String>,
    flag_shadow: std::marker::PhantomData<F>,
    value_shadow: std::marker::PhantomData<V>,
    spacer_shadow: std::marker::PhantomData<S>,
}
pub struct NoFlag;
pub struct WithValue;
pub struct WithFlag;
pub struct NoValue;
pub struct WithSpacer;
pub struct NoSpacer;

impl Arg<NoFlag, NoValue, NoSpacer> {
    pub fn new(flag: impl Into<String>) -> Arg<WithFlag, NoValue, NoSpacer> {
        Arg {
            flag: flag.into(),
            double_dash: false,
            without_dash: false,
            value_double_quote: false,
            value: None,
            value_spacer: None,
            flag_shadow: PhantomData,
            value_shadow: PhantomData,
            spacer_shadow: PhantomData,
        }
    }
}

impl Arg<WithFlag, NoValue, NoSpacer> {
    pub fn with_double_dash(mut self) -> Self {
        self.double_dash = true;
        self
    }

    pub fn without_dash(mut self) -> Self {
        self.without_dash = true;
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Arg<WithFlag, WithValue, NoSpacer> {
        self.value = Some(value.into());
        Arg {
            flag: self.flag,
            double_dash: self.double_dash,
            without_dash: self.without_dash,
            value_double_quote: self.value_double_quote,
            value: self.value,
            value_spacer: self.value_spacer,
            flag_shadow: PhantomData,
            value_shadow: PhantomData,
            spacer_shadow: PhantomData,
        }
    }

    pub fn value_with_vec(mut self, value: Vec<String>) -> Arg<WithFlag, WithValue, NoSpacer> {
        self.value = Some(value.join(""));
        Arg {
            flag: self.flag,
            double_dash: self.double_dash,
            without_dash: self.without_dash,
            value_double_quote: self.value_double_quote,
            value: self.value,
            value_spacer: self.value_spacer,
            flag_shadow: PhantomData,
            value_shadow: PhantomData,
            spacer_shadow: PhantomData,
        }
    }
}

impl Arg<WithFlag, WithValue, NoSpacer> {
    pub fn build(self) -> Vec<String> {
        let mut vals = vec![];

        match (self.double_dash, self.without_dash) {
            (true, false) => {
                vals.push(format!("--{}", self.flag));
                vals.push(self.value.unwrap());
            }
            (_, true) => {
                vals.push(self.flag);
                vals.push(self.value.unwrap());
            }
            _ => {
                vals.push(format!("-{}", self.flag));
                vals.push(self.value.unwrap());
            }
        }

        vals
    }

    pub fn with_value_spacer(
        mut self,
        spacer: impl Into<String>,
    ) -> Arg<WithFlag, WithValue, WithSpacer> {
        self.value_spacer = Some(spacer.into());
        Arg {
            flag: self.flag,
            double_dash: self.double_dash,
            without_dash: self.without_dash,
            value_double_quote: self.value_double_quote,
            value: self.value,
            value_spacer: self.value_spacer,
            flag_shadow: PhantomData,
            value_shadow: PhantomData,
            spacer_shadow: PhantomData,
        }
    }

    pub fn value_double_quote(mut self) -> Self {
        self.value_double_quote = true;
        self
    }
}

impl Arg<WithFlag, NoValue, NoSpacer> {
    pub fn build(self) -> Vec<String> {
        let mut vals = vec![];
        match (self.flag.clone(), self.double_dash, self.without_dash) {
            (flag, true, false) => vals.push(format!("--{}", flag)),

            (flag, _, false) => vals.push(format!("-{}", flag)),
            (flag, _, true) => vals.push(flag.to_string()),
        }
        vals
    }
}

impl Arg<WithFlag, WithValue, WithSpacer> {
    // NOTE : Normally with spacer there are together with the flag
    pub fn build(self) -> Vec<String> {
        let mut vals = Vec::new();
        match (self.double_dash, self.value_double_quote, self.without_dash) {
            (true, true, false) => {
                // Double quote means that it is a string
                vals.push(format!(
                    "\"--{}{}{}\"",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
            (true, false, false) => {
                vals.push(format!(
                    "--{}{}{}",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
            (_, true, true) => {
                vals.push(format!(
                    "\"{}{}{}\"",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
            (_, false, true) => {
                vals.push(format!(
                    "{}{}{}",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
            (false, true, false) => {
                vals.push(format!(
                    "\"-{}{}{}\"",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
            _ => {
                vals.push(format!(
                    "-{}{}{}",
                    self.flag,
                    self.value_spacer.unwrap(),
                    self.value.unwrap()
                ));
            }
        }
        vals
    }
}

#[cfg(test)]
mod test {
    use super::Arg;

    #[test]
    fn test_arg_builder() {
        let arg1 = Arg::new("arg").with_double_dash().build();

        assert_eq!(arg1, vec!["--arg".to_string()]);

        let arg1 = Arg::new("arg").build();

        assert_eq!(arg1, vec!["-arg".to_string()]);

        let arg1 = Arg::new("arg").value("sss").build();

        assert_eq!(arg1, vec!["-arg".to_string(), "sss".to_string()]);

        let arg1 = Arg::new("vf")
            .value(
                Arg::new(
                    Arg::new("scale")
                        .without_dash()
                        .value("1920x1080")
                        .with_value_spacer(":")
                        .build()
                        .join(""),
                )
                .without_dash()
                .value(
                    Arg::new("flags")
                        .without_dash()
                        .value("lanczos")
                        .with_value_spacer("=")
                        .build()
                        .join(""),
                )
                .with_value_spacer(":")
                .build()
                .join(""),
            )
            .value_double_quote()
            .with_value_spacer(" ")
            .build();

        println!("{:?}", arg1);
    }
}
