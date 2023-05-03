use godot::engine::Resource;
use godot::prelude::*;
use itertools::Itertools;
use std::str::FromStr;

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct ObjectScript {
    #[export]
    pub dynamic_objects: Array<Gd<ObjectData>>,
    #[export]
    pub static_objects: Array<Gd<ObjectData>>,
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl ObjectScript {}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct ObjectData {
    #[export]
    pub class_type: GodotString,
    #[export]
    pub resource_type: GodotString,
    #[export]
    pub name: GodotString,
    #[export]
    pub props: Dictionary,
    #[export]
    pub children: Array<Gd<ObjectData>>,
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl ObjectData {}

pub fn parse_game_object(contents: String) -> Gd<ObjectScript> {
    Gd::<ObjectScript>::with_base(|base| {
        let mut object_script = ObjectScript {
            dynamic_objects: Array::new(),
            static_objects: Array::new(),
            base,
        };

        let mut lines = contents
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter(|l| !l.starts_with('#'));

        while let Some(line) = lines.next() {
            match line {
                "DYNAMIC OBJECT START" => {
                    object_script.dynamic_objects.push(read_object(&mut lines))
                }
                "OBJECT START" => object_script.static_objects.push(read_object(&mut lines)),
                l => eprintln!("TODO: {}", l),
            };
        }

        object_script
    })
}

pub fn read_object<'s, I>(lines: &mut I) -> Gd<ObjectData>
where
    I: Iterator<Item = &'s str>,
{
    let class_type = lines
        .next()
        .unwrap()
        .strip_prefix("class type:")
        .unwrap()
        .trim()
        .trim_matches('"');
    let (resource_type, name) = lines
        .next()
        .unwrap()
        .splitn(2, ']')
        .map(|x| x.trim())
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    Gd::<ObjectData>::with_base(|base| {
        let mut object_data = ObjectData {
            class_type: class_type.into(),
            resource_type: resource_type
                .trim_start_matches('[')
                .trim_end_matches(']')
                .into(),
            name: name.trim_matches('"').into(),
            props: Dictionary::new(),
            children: Array::new(),
            base,
        };

        lines.next();
        loop {
            match lines.next().unwrap() {
                "}" => break,
                l => {
                    let (_, key, value) = l
                        .splitn(3, '"')
                        .map(|x| x.trim())
                        .collect_tuple::<(&str, &str, &str)>()
                        .unwrap();
                    let values = value
                        .split_whitespace()
                        .map(|s| f32::from_str(s).unwrap())
                        .collect_vec();
                    object_data.props.insert(
                        key,
                        match values.len() {
                            1 => values[0].to_variant(),
                            2 => Vector2 {
                                x: values[0],
                                y: values[1],
                            }
                            .to_variant(),
                            3 => Vector3 {
                                x: values[0],
                                y: values[1],
                                z: values[2],
                            }
                            .to_variant(),
                            _ => panic!(),
                        },
                    );
                }
            }
        }

        object_data
    })
}
