use serde_derive::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthRequestToken {
    pub token: String,
    pub secret: String,
    pub confirmed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthToken {
    pub token: String,
    pub secret: String,
}

impl OAuthRequestToken {
    pub fn from(raw: &str) -> Option<Self> {
        let map = query_split(raw)?;
        let token = map.get("oauth_token").map(|&x| String::from(x))?;
        let secret = map.get("oauth_token_secret").map(|&x| String::from(x))?;
        let confirmed = map.get("oauth_callback_confirmed").map(|&x| x == "true")?;

        Some(Self {
            token,
            secret,
            confirmed,
        })
    }
}

impl OAuthToken {
    pub fn from(raw: &str) -> Option<Self> {
        let map = query_split(raw)?;
        let token = map.get("oauth_token").map(|&x| String::from(x))?;
        let secret = map.get("oauth_token_secret").map(|&x| String::from(x))?;

        Some(Self { token, secret })
    }
}

fn query_split(str: &str) -> Option<HashMap<&str, &str>> {
    str.split('&')
        .into_iter()
        .map(|x| x.split('=').collect::<Vec<_>>())
        .try_fold(HashMap::new(), |mut map, vec| {
            let k = vec.get(0)?;
            let v = vec.get(1)?;
            map.insert(*k, *v);
            Some(map)
        })
}

#[cfg(test)]
#[test]
fn test_query_split() {
    use maplit::hashmap;

    let query = "hoge=abc&huga=xyz";
    let map = query_split(query);

    if let Some(map) = map {
        assert_eq!(map, hashmap! { "hoge" => "abc", "huga" => "xyz" })
    } else {
        panic!()
    }
}
