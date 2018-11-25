use flow::store::StoreServiceTrait;
use std::rc::Rc;
use super::*;

pub trait ParallelServiceTrait {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()>;
    fn do_parallel_task(&self, instances: ParallelBatchInstance, carrier: RawTask);
}

pub struct ParallelServiceImpl {
    pub delivery_svc: Rc<TaskServiceTrait>,
    pub delivery_dao: Rc<TaskDaoTrait>,
    pub store: Rc<StoreServiceTrait>,
}

impl ParallelServiceTrait for ParallelServiceImpl {
    fn parallel(&self, batch: ParallelBatchInstance) -> Result<()> {
        let raw = RawTask::new(&batch, &batch.thing.key, TaskType::ParallelBatch as i16)?;
        match self.delivery_dao.insert(&raw) {
            Ok(_carrier) => {
                // to process asynchronous
                let _ = CHANNEL_PARALLEL.sender.lock().unwrap().send((batch, raw));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn do_parallel_task(&self, batch: ParallelBatchInstance, carrier: RawTask) {
        let mut tasks: Vec<RawTask> = Vec::new();
        let mut tuple: Vec<(StoreTaskInfo, RawTask)> = Vec::new();
        for instance in batch.instances.iter() {
            match self.store.generate_store_task(instance) {
                Ok(task) => {
                    match RawTask::new(&task, &instance.thing.key, TaskType::Store as i16) {
                        Ok(car) => {
                            tasks.push(car.clone());
                            tuple.push((task, car))
                        }
                        Err(e) => {
                            error!("{}", e);
                            return;
                        }
                    }
                }
                // any error will break the process
                _ => return
            }
        }
        if let Ok(_) = self.delivery_svc.create_batch_and_finish_carrier(&tasks, &carrier.task_id) {
            for c in tuple {
                let _ = CHANNEL_STORE.sender.lock().unwrap().send(c);
            }
        }
    }
}

