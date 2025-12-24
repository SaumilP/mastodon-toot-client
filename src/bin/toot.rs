// file responsible for sending toot to Mastodon social site...

use mammut::{Data, Mastodon, Registration, status_builder::StatusBuilder, apps::{AppBuilder, Scopes}};
use mastodon_toot_client2::random_location;
use dotenv;

fn main() {
    dotenv::dotenv().ok();
    if ::std::env::var("CLIENT_ID").is_ok() {
    	println!(">>>>> About to toot using `.env` file ... <<<<<<<<");
        from_configuration();
    } else {
    	println!(">>>>> About to toot from new client registration ... <<<<<<<<");
        register_and_post();
    }
}

#[allow(dead_code)]
fn register_and_post() {
    let app = AppBuilder {
        client_name: "rust-client",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::ReadWrite,
        website: None,
    };

	// Register the app using configuration in `.env` file
    let mut registration = Registration::new(env("BASE"));
    match registration.register(app) {
        Ok(_) => println!("App registered successfully"),
        Err(e) => panic!("Register failed: {}", e),
    }
    let url = match registration.authorise() {
        Ok(url) => url,
        Err(e) => panic!("Registration authorize failed: {}", e),
    };

    println!("Please visit {}, authorise and enter the code it gives you: ", url);

	// get token code and use that for posting new status...
	
	// FIXME Below code reads code entered by user, find a better way to do website scrapping if required
    let mut code = String::new();
    match std::io::stdin().read_line(&mut code) {
        Ok(_) => {},
        Err(e) => panic!("Reading code failed: {}", e),
    }
    
    // create the app client
    let mastodon = match registration.create_access_token(code.trim().to_string()) {
        Ok(m) => m,
        Err(e) => panic!("Creating access token failed: {}", e),
    };
    println!("{:#?}", mastodon.data);
    
    // Lets update status
    let status = random_location();
    let sb = StatusBuilder::new(status);
    println!("StatusBuilder = {:#?}", sb);
    
    match mastodon.new_status(sb) {
        Ok(_) => println!("Status posted successfully"),
        Err(e) => panic!("Could not post status: {}", e),
    }
    
    println!("Done!");
}

#[allow(dead_code)]
fn from_configuration() {
    let data = Data {
        base: env("BASE").into(),
        client_id: env("CLIENT_ID").into(),
        client_secret: env("CLIENT_SECRET").into(),
        redirect: String::from("urn:ietf:wg:oauth:2.0:oob").into(),
        token: env("TOKEN").into()
    };

	// create mastodon client from configured data
    let mastodon: Mastodon = Mastodon::from_data(data);

	// lets get new random status
    let status = crate::random_location();
    println!("Posting {}", status);

	// post status to mastodon
    let sb = StatusBuilder::new(status);
    println!("StatusBuilder = {:#?}", sb);
    match mastodon.new_status(sb) {
        Ok(_) => println!("Status posted successfully"),
        Err(e) => panic!("Could not post status: {}", e),
    }

    println!("Done!");
}

fn env(s: &str) -> String {
    ::std::env::var(s).unwrap_or_else(|_| {
    		panic!("must have `{}` definded", s)
    })
}
