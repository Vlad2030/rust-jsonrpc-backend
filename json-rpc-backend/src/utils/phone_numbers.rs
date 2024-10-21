use regex;

pub struct PhoneParser {
    raw: String,
    phone_codes: Vec<u32>,
    regexps: Vec<regex::Regex>,
}

impl PhoneParser {
    pub fn new(raw: String) -> Self {
        let phone_codes = vec![982, 986, 912, 934];
        let regexps = vec![
            regex::Regex::new(r"^\+7 (\d{3}) (\d{3}) (\d{2})(\d{2})$").unwrap(),
            regex::Regex::new(r"^\+7 (\d{3}) (\d{3}) (\d{2}) (\d{2})$").unwrap(),
            regex::Regex::new(r"^\+7 \((\d{3})\) (\d{3})-(\d{2})(\d{2})$").unwrap(),
            regex::Regex::new(r"^\+7(\d{3})(\d{3})(\d{2})(\d{2})$").unwrap(),
            regex::Regex::new(r"^8 (\d{3}) (\d{3}) (\d{2})(\d{2})$").unwrap(),
            regex::Regex::new(r"^8 (\d{3}) (\d{3}) (\d{2}) (\d{2})$").unwrap(),
            regex::Regex::new(r"^8 \((\d{3})\) (\d{3})-(\d{2})(\d{2})$").unwrap(),
            regex::Regex::new(r"^8(\d{3})(\d{3})(\d{2})(\d{2})$").unwrap(),
        ];

        Self { raw, phone_codes, regexps }
    }

    pub fn parse(&self) -> Option<String> {
        if let Some(captures) = self.match_phone() {
            let code: u32 = captures[1].parse().unwrap_or(0);
            if !self.phone_codes.contains(&code) {
                return None
            }

            let formatted_phone = format!(
                "+7-{}-{}-{}{}",
                &captures[1], &captures[2], &captures[3], &captures[4]
            );
            return Some(formatted_phone)
        }
        None
    }

    fn match_phone(&self) -> Option<regex::Captures> {
        for regex in &self.regexps {
            if let Some(captures) = regex.captures(&self.raw) {
                return Some(captures)
            }
        }
        None
    }
}
