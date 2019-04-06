extern crate chrono;
extern crate log;
extern crate nature;
extern crate nature_common;
extern crate nature_db;
extern crate rocket;
extern crate serde_json;

use std::thread;
use std::time;

use common::*;
use nature::system::sys_init;
use nature_common::*;
use nature_db::*;

use self::rocket::http::ContentType;

mod common;

#[test]
fn web_post_and_save_one() {
    let threads = sys_init();
    println!("created threads: {:?}", threads);
    println!("------------------ insert thing define -----------------");
    let key = "/save_one".to_string();
    let _ = ThingDefineDaoImpl::new_by_key(&key);
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing = Thing::new(&key).unwrap();
    let json = serde_json::to_string(&(
        instance)).unwrap();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let mut will_del = instance.clone();
    will_del.id = 50086162352625018456971694157959652318;
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("delete {} rows", x);
    }
    println!("------------------ submit new instance -----------------");
    let client = get_test_client();
    let mut response = client.post("/input")
        .body(json)
        .header(ContentType::JSON)
        .dispatch();

    thread::sleep(time::Duration::from_millis(500));
    println!("------------------ verify -----------------");
    // check return result
    let rtn = response.body_string().unwrap();
    println!("{:?}", rtn);
    assert_eq!(rtn, r#"{"Ok":50086162352625018456971694157959652318}"#);
    // get instance which is saved to db
    let i_d = InstanceDaoImpl {};
    let _ins_db = i_d.get_by_id(50086162352625018456971694157959652318).unwrap().unwrap();
}

