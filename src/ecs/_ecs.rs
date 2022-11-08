
pub struct Ecs<'a, 'b, T>
where
    T: EcsContent<IntoIter = (&'static str, Value)>,
{
    ts: Timestamp,
    message: Option<String>,
    //labels: HashMap<String, String>,
    tags: HashSet<String>,
    file: Option<&'a PosixFile>,
    macb: Option<&'a Macb>,
    content: &'b T,
}

impl<'a, 'b, T> Ecs<'a, 'b, T>
where
    T: EcsContent<IntoIter = (&'static str, Value)>,
{
    pub fn new(ts: Timestamp, content: &'b T) -> Self {
        Self {
            ts,
            message: None,
            //labels: HashMap::new(),
            tags: HashSet::new(),
            file: None,
            macb: None,
            content,
        }
    }

    pub fn with_additional_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_owned());
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_file ] [ file ] [ PosixFile ];
      [ with_macb ] [ macb ] [ Macb ];
    )]
    pub fn method(mut self, ts: &'a ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}

impl<T> From<Ecs<'_, '_, T>> for Value
where
    T: EcsContent<IntoIter = (&'static str, Value)>,
{
    fn from(val: Ecs<T>) -> Value {
        let mut m = HashMap::from([
            (
                "@timestamp",
                Value::Number(val.ts.timestamp_millis().into()),
            ),
            ("ecs", json!({"version": "8.4"})),
        ]);
        m.extend(val.content);

        if let Some(message) = val.message {
            m.insert("message", json!(message));
        }

        if !val.tags.is_empty() {
            m.insert("tags", json!(val.tags));
        }

        if let Some(file) = val.file.as_ref() {
            let mut file_map: HashMap<&str, Value> = (*file).into();
            if let Some(macb) = val.macb {
                let macb_short: String = macb.into();
                let macb_long: Vec<&str> = macb.into();
                file_map.insert("macb_short", json!(macb_short));
                file_map.insert("macb_long", json!(macb_long));
            }

            m.insert("file", json!(file_map));
        }

        json!(m)
    }
}
