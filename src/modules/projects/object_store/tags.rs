use url::form_urlencoded::Serializer;

#[derive(Debug,Clone,Default,Eq,PartialEq)]
pub struct TagSet(String);

impl TagSet {

    // 往 字符串里面塞数据，将 url 编码后的数据塞进去
    pub fn push(&mut self, key: &str, value: &str) {
        Serializer::new(
            &mut self.0
        ).append_pair(key, value);
    }

    pub fn encoded(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_tag_set() {
        let mut set = TagSet::default();
        set.push("test/foo", "value sdlks");
        set.push("foo", " sdf _ /+./sd");
        assert_eq!(
            set.encoded(),
            "test%2Ffoo=value+sdlks&foo=+sdf+_+%2F%2B.%2Fsd"
        );
    }

}