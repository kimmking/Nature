//#![feature(plugin)]
//extern crate chrono;
//extern crate log;
//extern crate nature;
//extern crate nature_common;
//extern crate nature_db;
//extern crate rocket;
//extern crate rocket_contrib;
//extern crate serde_json;
//
//use std::thread;
//use std::time;
//
//use nature::system::*;
//use nature_common::*;
//use nature_db::*;
//
//use self::nature::task::*;
//
//mod common;


/// build  `nature_integrate_test_converter` first
#[test]
fn local_converter() {
    // TODO
//    let _ = sys_init();
//    println!("------------------ insert thing define -----------------");
//    let from = "/local_converter/from";
//    let to = "/local_converter/to";
//    let _ = ThingDefineDaoImpl::new_by_key(from);
//    let _ = ThingDefineDaoImpl::new_by_key(to);
//    let url = r#"nature_integrate_test_converter.dll:rtn_one"#;
//    let _ = OneStepFlowDaoImpl::delete_by_biz(from, to);
//    let _ = OneStepFlowDaoImpl::insert_by_biz(from, to, &url, "LocalRust");
//    println!("------------------ prepare instance to submit -----------------");
//    // prepare input para
//    let mut instance = Instance::default();
//    instance.data.thing = Thing::new(from).unwrap();
//    println!("------------------ remove existed instance -----------------");
//    // remove if instance exists
//    let will_del = instance.clone();
//    if let Ok(x) = InstanceDaoImpl::delete(&will_del) {
//        println!("deleted : {} row", x);
//    }
//    println!("------------------ submit new instance -----------------");
//    let _rtn = IncomeController::input(instance);
//    thread::sleep(time::Duration::from_millis(1000));
//    println!("------------------ verify -----------------");
//    // get instance which is saved to db
//    let dao = InstanceDaoImpl {};
//    let _ins_db = dao.get_by_id(302766851427585644820594078352365831400).unwrap().unwrap();
}

