use alloc::borrow::Cow;
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Attribute {
    /// 在常规的 HTTP 应答中，Content-Disposition
    /// 响应标头指示回复的内容该以何种形式展示，是以内联
    /// 的形式（即网页或者页面的一部分），还是以附件的形式下
    /// 载并保存到本地。
    ContentDisposition,

    /// 表示标头 Content-Encoding 列出了对当前应用资源
    /// 的任何编码类型， 以及编码的顺序。它让接收者知道需要
    /// 以何种顺序解码数据以获得 Content-Type 标头中描述
    /// 的原始内容格式。内容编码主要用于在不丢失原媒体类型内
    /// 容的情况下压缩内容。
    ContentEncoding,

    /// HTTP Content-Language 表示标头用来说明访问者希望
    /// 采用的语言，这样的话用户就可以根据自己偏好的语言来定制不同的内容。
    ContentLanguage,

    /// Content-Type 表示标头用于指示资源的原始媒体
    /// 类型（在发送时应用任何内容编码之前）。
    ContentType,

    /// Cache-Control 通用消息头字段，被用于在 http 请求和
    /// 响应中，通过指定指令来实现缓存机制。缓存指令是单向的，
    /// 这意味着在请求中设置的指令，不一定被包含在响应中。
    CacheControl,

    /// 指定对象的存储类别
    StorageClass,

    /// 用户指定元数据
    Metadata(Cow<'static, str>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct AttributeValue(Cow<'static, str>);

/// 为 AttributeValue 一些 trait
/// 方便处理的

impl AsRef<str> for AttributeValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for AttributeValue {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl Deref for AttributeValue {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0.as_ref()
    }
}

/// 封装一个 hashmap 存放数据
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Attributes(HashMap<Attribute, AttributeValue>);

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn insert(&mut self, key: Attribute, value: AttributeValue) -> Option<AttributeValue> {
        self.0.insert(key, value)
    }

    pub fn get(&self, key: &Attribute) -> Option<&AttributeValue> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &Attribute) -> Option<AttributeValue> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> AttributesIter<'_> {
        self.into_iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K, V> FromIterator<(K, V)> for Attributes
where
    K: Into<Attribute>,
    V: Into<AttributeValue>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = (&'a Attribute, &'a AttributeValue);
    type IntoIter = AttributesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AttributesIter(self.0.iter())
    }
}

/// 为 AttributesIter 实现 迭代器
#[derive(Debug)]
pub struct AttributesIter<'a>(std::collections::hash_map::Iter<'a, Attribute, AttributeValue>);

impl<'a> Iterator for AttributesIter<'a> {
    type Item = (&'a Attribute, &'a AttributeValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_value() {
        let v1 = AttributeValue::from("test");
        let v2 = AttributeValue::from(String::from("test"));
        assert_eq!(v1.as_ref(), "test");
        assert_eq!(v2.as_ref(), "test");
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_map_iter() {}
}
