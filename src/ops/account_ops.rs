use jammdb::{Error, DB};

use crate::args::account_args::{
    AccountAuth, AccountCommand, AccountSubcommand, DeleteAccount, ShowAccount, UserAccount,
};

pub fn handle_account_command(account: AccountCommand) {
    let command = account.command;
    match command {
        AccountSubcommand::AuthLogin(account_auth) => {
            let result = auth_login(account_auth);
            println!("Bearer token is: {:?}", result.unwrap());
        }
        AccountSubcommand::Save(account) => {
            let result = save_account(&account);
            println!("Saved account is: {:?}", result.unwrap());
        }
        AccountSubcommand::Delete(account) => {
            let result = delete_account(&account);
            println!("Deleted account is: {:?}", result.unwrap());
        }
        AccountSubcommand::Update(account) => {}
        AccountSubcommand::Show(account) => {
            let result = read_account(account);
            println!("Account is: {:?}", result.unwrap());
        }
    }
}

pub fn auth_login(account_auth: AccountAuth) -> Result<String, ureq::Error> {
    let username = account_auth.username;
    let password = account_auth.password;
    let twofa_code = account_auth.twofa_code;

    let resp: String = ureq::post("https://api2.hiveos.farm/api/v2/auth/login")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "login": username,
            "password": password,
            "twofa_code": twofa_code,
            "remember": true
        }))?
        .into_string()?;

    Ok(resp)
}

pub fn delete_account(_account: &DeleteAccount) -> Result<(), Error> {
    let db_name = "/tmp/hiveos_pleb_cli.db";

    // open a new DB and write the account info to it
    print!(r#"Opening the database"#);
    let db = DB::open(db_name)?;

    // open a writable transaction so we can make changes
    let tx = db.tx(true)?;
    let b = tx.get_or_create_bucket("accounts")?;

    // b.delete(&account.to_be, account_bytes)?;
    tx.commit()?;
    Ok(())
}

pub fn save_account(account: &UserAccount) -> Result<(), Error> {
    let db_name = "/tmp/hiveos_pleb_cli.db";

    // open a new DB and write the account info to it
    print!(r#"Opening the database"#);
    let db = DB::open(db_name)?;

    // open a writable transaction so we can make changes
    let tx = db.tx(true)?;

    print!(
        r#"Creating bucket to store account {:#?}"#,
        &account.username
    );
    // create a bucket to store accounts
    let accounts_bucket = tx.get_or_create_bucket("accounts")?;
    let account_bytes = rmp_serde::to_vec(&account).unwrap();

    print!(r#"Save the account:  {:#?}"#, &account.username);
    accounts_bucket.put(&*account.username, account_bytes)?;

    // commit the changes so we can save them to disc
    print!(
        r#"Committing the transaction for:  {:#?}"#,
        &account.username
    );
    tx.commit()?;

    Ok(())
}

pub fn read_account(account: ShowAccount) -> Result<(), Error> {
    let db = DB::open("/tmp/hiveos_pleb_cli.db")?;

    let tx = db.tx(false)?;

    let accounts_bucket = tx.get_bucket("accounts")?;

    if let Some(data) = accounts_bucket.get(&account.username) {
        if data.is_kv() {
            let kv = data.kv();

            // deserialize into a user struct
            let db_user: UserAccount = rmp_serde::from_slice(kv.value()).unwrap();
            print!("User email is: {:?}", &db_user.email);
            print!("User name is: {:?}", &db_user.username);
            print!("User account is: {:?}", &db_user.account_token);
        }
    }
    Ok(())
}

pub fn get_account_bearer_token(username: String) -> Result<UserAccount, jammdb::Error> {
    let db = DB::open("/tmp/hiveos_pleb_cli.db")?;

    let tx = db.tx(false)?;

    let accounts_bucket = tx.get_bucket("accounts")?;

    if let Some(data) = accounts_bucket.get(&username) {
        if data.is_kv() {
            let kv = data.kv();
            // deserialize into a user struct
            let db_user: UserAccount = rmp_serde::from_slice(kv.value()).unwrap();
            Ok(db_user)
        } else {
            Err(jammdb::Error::KeyValueMissing)
        }
    } else {
        Err(jammdb::Error::BucketMissing)
    }
}
