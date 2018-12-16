#![feature(plugin)]
extern crate chrono;
extern crate log;
extern crate nature;
extern crate nature_common;
extern crate nature_db;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;

use std::thread;
use std::time;

use common::*;
use nature::system::*;
use nature_common::Instance;
use nature_db::*;

use self::nature::flow::*;

mod common;


/// build  `nature_integrate_test_converter` first
#[test]
fn local_converter() {
    let _ = sys_init();
    println!("------------------ insert thing define -----------------");
    let from = "/local_converter/from";
    let to = "/local_converter/to";
    new_thing_define(from);
    new_thing_define(to);
    let url = r#"../../../Nature-integrate-test-converter/target/debug/nature_integrate_test_converter.dll:rtn_one"#;
    one_step_flow_delete(from, to);
    new_one_step_flow(from, to, &url, "LocalRust");
    println!("------------------ prepare instance to submit -----------------");
    // prepare input para
    let mut instance = Instance::default();
    instance.data.thing.key = from.to_string();
    println!("------------------ remove existed instance -----------------");
    // remove if instance exists
    let will_del = instance.clone();
    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
        println!("deleted : {} row", x);
    }
    println!("------------------ submit new instance -----------------");
    let _rtn = IncomeController::input(instance);
    thread::sleep(time::Duration::from_millis(1000));
    println!("------------------ verify -----------------");
    // get instance which is saved to db
    let i_d = InstanceDaoImpl {};
    let _ins_db = i_d.get_by_key("/local_converter/to", 217789594388339757346716979317903552035).unwrap().unwrap();
}

