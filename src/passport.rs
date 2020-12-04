use crate::error::{AocError, Result};
use std::collections::HashMap;

pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    pub fn from_input(input: &Vec<String>) -> Vec<Result<Self>> {
        let mut passports = Vec::new();
        let mut acc = Vec::new();
        for line in input {
            if line.len() == 0 {
                let data: String = acc.join(" ");
                passports.push(Passport::new(&data));
                acc = Vec::new();
            } else {
                acc.push(line.clone());
            }
        }

        if acc.len() > 0 {
            let data: String = acc.join(" ");
            passports.push(Passport::new(&data));
        }

        passports
    }

    pub fn new(data: &str) -> Result<Self> {
        let mut fields: HashMap<&str, String> = HashMap::new();

        for info in data.split(" ").map(|field| field.split(":")) {
            let info: Vec<&str> = info.collect();

            if info.len() != 2 {
                return Err(AocError::PassportInfoError(data.to_string()));
            }

            fields.insert(info[0], info[1].to_string());
        }

        Ok(Passport {
            byr: fields
                .get("byr")
                .ok_or(AocError::PassportInvalid("byr".to_string()))?
                .clone(),
            iyr: fields
                .get("iyr")
                .ok_or(AocError::PassportInvalid("iyr".to_string()))?
                .clone(),
            eyr: fields
                .get("eyr")
                .ok_or(AocError::PassportInvalid("eyr".to_string()))?
                .clone(),
            hgt: fields
                .get("hgt")
                .ok_or(AocError::PassportInvalid("hgt".to_string()))?
                .clone(),
            hcl: fields
                .get("hcl")
                .ok_or(AocError::PassportInvalid("hcl".to_string()))?
                .clone(),
            ecl: fields
                .get("ecl")
                .ok_or(AocError::PassportInvalid("ecl".to_string()))?
                .clone(),
            pid: fields
                .get("pid")
                .ok_or(AocError::PassportInvalid("pid".to_string()))?
                .clone(),
            cid: match fields.get("cid") {
                Some(val) => Some(val.clone()),
                None => None,
            },
        })
    }

    fn validate_birth_year(&self) -> Result<()> {
        let byr = self.byr.parse::<u32>()?;
        if byr < 1920 || byr > 2002 {
            return Err(AocError::PassportInvalid("byr".to_string()));
        }
        Ok(())
    }

    fn validate_issue_year(&self) -> Result<()> {
        let iyr = self.iyr.parse::<u32>()?;
        if iyr < 2010 || iyr > 2020 {
            return Err(AocError::PassportInvalid("iyr".to_string()));
        }
        Ok(())
    }

    fn validate_expiration_year(&self) -> Result<()> {
        let eyr = self.eyr.parse::<u32>()?;
        if eyr < 2020 || eyr > 2030 {
            return Err(AocError::PassportInvalid("eyr".to_string()));
        }
        Ok(())
    }

    fn validate_height(&self) -> Result<()> {
        if self.hgt.ends_with("cm") {
            let height = self.hgt.replace("cm", "").parse::<u32>()?;
            if height < 150 || height > 193 {
                return Err(AocError::PassportInvalid("hgt".to_string()));
            }
        } else if self.hgt.ends_with("in") {
            let height = self.hgt.replace("in", "").parse::<u32>()?;
            if height < 59 || height > 76 {
                return Err(AocError::PassportInvalid("hgt".to_string()));
            }
        } else {
            return Err(AocError::PassportInvalid("hgt".to_string()));
        }

        Ok(())
    }

    fn validate_hair_color(&self) -> Result<()> {
        if !self.hcl.starts_with("#") || self.hcl.len() != 7 {
            return Err(AocError::PassportInvalid("hcl".to_string()));
        }

        if self.hcl.chars().filter(|c| c.is_ascii_hexdigit()).count() < 6 {
            return Err(AocError::PassportInvalid("hcl".to_string()));
        }

        Ok(())
    }

    fn validate_eye_color(&self) -> Result<()> {
        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
            _ => Err(AocError::PassportInvalid("ecl".to_string())),
        }
    }

    fn validate_pid(&self) -> Result<()> {
        if self.pid.len() != 9 || self.pid.chars().filter(|ch| ch.is_numeric()).count() != 9 {
            return Err(AocError::PassportInvalid("pid".to_string()));
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_birth_year()?;
        self.validate_issue_year()?;
        self.validate_expiration_year()?;
        self.validate_height()?;
        self.validate_hair_color()?;
        self.validate_eye_color()?;
        self.validate_pid()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_missing_fields_on_construction() {
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(Passport::new(data).is_ok());

        let data = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        assert!(Passport::new(data).is_err());
    }

    #[test]
    fn making_passports_from_input() {
        let input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ];

        let res = Passport::from_input(&input);
        assert_eq!(res.len(), 4);
    }

    #[test]
    fn invalid_passports() {
        let input = vec![
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "".to_string(),
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
            "".to_string(),
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "".to_string(),
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ];

        let res = Passport::from_input(&input)
            .into_iter()
            .collect::<Result<Vec<Passport>>>()
            .expect("Could not construct passports");

        assert_eq!(res.len(), 4);

        for passport in res {
            assert!(passport.validate().is_err());
        }
    }

    #[test]
    fn valid_passports() {
        let input = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ];

        let res = Passport::from_input(&input)
            .into_iter()
            .collect::<Result<Vec<Passport>>>()
            .expect("Could not construct passports");

        assert_eq!(res.len(), 4);

        for passport in res {
            passport.validate().expect("passport invalid");
        }
    }
}
