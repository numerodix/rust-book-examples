mod business {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Company {
        // department -> members
        people: HashMap<String, Vec<String>>,
    }

    impl Company {
        pub fn new() -> Self {
            Company{
                people: HashMap::new(),
            }
        }

        pub fn add_person(&mut self, department: &str, person: &str) {
            let dept = String::from(department);
            let individual = String::from(person);

            let members = self.people.entry(dept).or_insert(vec![]);
            members.push(individual);
        }

        pub fn get_people_in_department(&self, department: &str) -> Option<Vec<&str>> {
            match self.people.contains_key(department) {
                false => None,
                true => {
                    let members = self.people.get(department).unwrap();

                    let mut retval: Vec<&str> = members.iter().map(|person| &person[..]).collect();
                    retval.sort();

                    Some(retval)
                }
            }
        }

        pub fn get_everyone_by_department(&self) -> Vec<(&str, Vec<&str>)> {
            let mut retval = vec![];

            let mut departments: Vec<&String> = self.people.keys().collect();
            departments.sort();

            for dept in &departments {
                let members = self.people.get(*dept).unwrap();

                let mut members_refs: Vec<&str> = members.iter().map(|person| &person[..]).collect();
                members_refs.sort();

                let pair = (&dept[..], members_refs);
                retval.push(pair);
            }

            retval
        }

        pub fn pp(&self) {
            println!("{:#?}", self.people);
        }
    }

}


struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x
    }
}


fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}", p.x());



    let mut company = business::Company::new();

    company.add_person("Engineering", "Sally");
    company.add_person("Engineering", "Amy");
    company.add_person("Sales", "Gunther");
    company.add_person("Sales", "Amir");
    company.add_person("HR", "Massimo");


    let not_exist = company.get_people_in_department("Ninjas");
    assert_eq!(not_exist, None);

    let sellers = company.get_people_in_department("Sales").unwrap();
    let exp = vec!["Amir", "Gunther"];
    assert_eq!(sellers, exp);


    let everyone = company.get_everyone_by_department();
    let exp = vec![
        ("Engineering", vec!["Amy", "Sally"]),
        ("HR", vec!["Massimo"]),
        ("Sales", vec!["Amir", "Gunther"]),
    ];
    assert_eq!(everyone, exp);


    company.pp();
}
