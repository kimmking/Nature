use nature_common::{ConverterParameter, ConverterReturned, Instance, Protocol};
use nature_db::{Mission, RawTask};
use nature_db::flow_tool::state_check;

use crate::builtin_executor::BuiltIn;
use crate::task::{http_execute_async, local_execute, TaskForConvert};

pub type Execute = fn(para: &ConverterParameter) -> ConverterReturned;

pub async fn gen_and_call_out(task: &TaskForConvert, raw: &RawTask, mission: &Mission, last_target: &Option<Instance>, master: Option<Instance>) -> ConverterReturned {
    if let Some(ref last) = last_target {
        if let Some(demand) = &mission.states_demand {
            if !state_check(&last.states, &demand.need_none, &demand.need_all, &demand.need_any) {
                return ConverterReturned::None;
            }
        }
    };

    let para = ConverterParameter {
        from: task.from.clone(),
        last_state: last_target.clone(),
        task_id: raw.task_id.clone(),
        master,
        cfg: mission.executor.settings.to_string(),
    };
    debug!("execute: from: {}, to : {}, executor: {}", task.from.meta, task.target.to.meta_string(), &mission.executor.url);
    let rtn = match &mission.executor.protocol {
        Protocol::Http => http_execute_async(&mission.executor.url, &para).await,
        Protocol::LocalRust => local_execute(&mission.executor.url, para).await,
        Protocol::BuiltIn => match BuiltIn::get(&mission.executor.url) {
            Ok(exe) => exe(&para),
            Err(_) => ConverterReturned::LogicalError("get built-in executor failed".to_string())
        }
        _ => ConverterReturned::LogicalError(format!("Did not implement for protocal : {:?}", &mission.executor.protocol)),
    };
    rtn
}