use super::*;
use uuid::Uuid;

#[test]
fn thing_instance_without_id() {
    let th = Instance {
        id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
        data: InstanceNoID{
            thing: Thing {
                key: String::new(),
                version: 0,
            },
            execute_time: 0,
            create_time: 0,
            content: String::new(),
            context: String::new(),
        },
    };
    println!("{:?}", Uuid::from_bytes(&th.id));
    let id = th.id.into_iter().all(|x| *x == 0);
    assert_eq!(id, true);
    assert_eq!(th.id.len(), 16);
}