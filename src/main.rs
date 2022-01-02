use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::{self, to_writer_pretty, Map};
use std::{env, io::BufWriter, fs::File};
use dotenv;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
    created_at: i64,
}


#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct Language {
    id: u32,
    name: String,
    identifier: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct Campus {
        id: Option<i32>,
        name: Option<String>,
        time_zone: Option<String>,
        language: Option<Language>,
        users_count: Option<i32>,
        vogsphere_id: Option<i32>,
        country: Option<String>,
        address: Option<String>,
        zip: Option<String>,
        city: Option<String>,
        website: Option<String>,
        facebook: Option<String>,
        twitter: Option<String>,
        active: Option<bool>,
        email_extension: Option<String>,
        default_hidden_phone: Option<bool>,
        endpoint: Option<i32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct CampusInfo {
    items: Vec<Campus>,
}


async fn init_session() -> Result<AccessToken, reqwest::Error> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id = env::var("client_id").unwrap();
    let client_secret = env::var("client_secret").unwrap();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client
        .post("https://api.intra.42.fr/oauth/token")
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok~~");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };
    let token = response.json::<AccessToken>().await?;
    Ok(token)
}

fn jsonize(text: &str) -> Result<CampusInfo, serde_json::Error> {
    let camp: CampusInfo = serde_json::from_str::<CampusInfo>(text)?;
    Ok(camp)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let acc_token = init_session().await?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/v2/campus")
        .header(AUTHORIZATION, format!("Bearer {}", acc_token.access_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok~~");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

    // let tmp = response.text().await?;
    // let camp: CampusInfo = jsonize(tmp.as_str()).unwrap();
    // println!("{:?}", camp);

    /*
   let john =  r#"
   {
       "id":55,
       "name":"Tétouan",
       "time_zone":"Africa/Casablanca",
       "language":
       {
           "id":1,
           "name":"Français",
           "identifier":"fr"
        },
        "users_count":1,
        "vogsphere_id":null,
        "country":"Morocco",
        "address":"Parc Tétouan Shore, CP93150 , Martil, Tétouan",
        "zip":"93000",
        "city":"Tétouan",
        "website":"https://1337.ma",
        "facebook":"",
        "twitter":"https://twitter.com/1337FIL",
        "active":true,
        "email_extension":"1337.ma",
        "default_hidden_phone":false,
        "endpoint":null
    }"#;

    let tmp: Campus = serde_json::from_str(john).unwrap();
    let writer = BufWriter::new(File::create("res.json").unwrap());
    serde_json::to_writer_pretty(writer, &tmp).unwrap();
    println!("{:?}", tmp);
    */

    Ok(())
}
