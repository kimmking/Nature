use rpc::*;
use std::marker::PhantomData;
use super::*;

pub trait ConvertTaskTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()>;
    fn do_convert_task(carrier: Carrier<ConverterInfo>);
}

pub struct ConvertTaskImpl<SP, SD> {
    plan: PhantomData<SP>,
    delivery: PhantomData<SD>,
}

impl<SP, SD> ConvertTaskTrait for ConvertTaskImpl<SP, SD> where SP: PlanServiceTrait, SD: DeliveryServiceTrait {
    fn submit_callback(delayed: DelayedInstances) -> Result<()> {
        let carrier = TableDelivery::get::<ConverterInfo>(delayed.carrier_id)?;
        match delayed.result {
            CallbackResult::Err(err) => {
                let err = NatureError::ConverterLogicalError(err);
                SD::move_to_err(err, carrier);
                Ok(())
            }
            CallbackResult::Instances(ins) => Self::handle_instances(&carrier, &ins)
        }
    }
    fn do_convert_task(carrier: Carrier<ConverterInfo>) {
        let para = CallOutParameter::new(&carrier);
        let _ = match ConvertImpl::convert(para) {
            Ok(ConverterReturned::Instances(instances)) => {
                match Self::handle_instances(&carrier, &instances) {
                    Ok(_) => (),
                    Err(NatureError::DaoEnvironmentError(_)) => (),
                    Err(err) => {
                        SD::move_to_err(err, carrier.clone());
                    }
                }
            }
            Ok(ConverterReturned::Delay(delay)) => {
                let _ = TableDelivery::update_execute_time(carrier.id, carrier.execute_time + delay as i64);
                ()
            }
            Err(err) => match err {
                // only **Environment Error** will be retry
                NatureError::ConverterEnvironmentError(_) => (),
                // other error will drop into error
                _ => SD::move_to_err(err, carrier)
            }
        };
    }
}

impl<SP, SD> ConvertTaskImpl<SP, SD> where SP: PlanServiceTrait, SD: DeliveryServiceTrait {
    fn handle_instances(carrier: &Carrier<ConverterInfo>, instances: &Vec<Instance>) -> Result<()> {
// check status version to avoid loop
        let instances = verify(&carrier.mapping.to, &instances)?;
        let plan = SP::new(&carrier.content.data, &instances)?;
        Self::to_store(carrier, plan);
        Ok(())
    }
    fn to_store(carrier: &Carrier<ConverterInfo>, plan: PlanInfo) {
        let store_infos: Vec<StoreInfo> = plan.plan.iter().map(|instance| {
            StoreInfo {
                instance: instance.clone(),
                converter: Some(carrier.content.data.clone()),
            }
        }).collect();
        let new_tasks = SD::create_batch_and_finish_carrier(
            store_infos,
            carrier.to_owned(),
            carrier.mapping.to.key.clone(),
            DataType::Convert as u8,
        );
        if new_tasks.is_err() {
            return;
        }
        for task in new_tasks.unwrap() {
            SD::send_carrier(&CHANNEL_STORE.sender, task)
        }
    }
}


fn verify(to: &Thing, instances: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut rtn: Vec<Instance> = Vec::new();

    // only one status instance should return
    let define = ThingDefineCacheImpl::get(to)?;
    if define.is_status() {
        if instances.len() > 1 {
            return Err(NatureError::ConverterLogicalError("[status thing] must return less 2 instances!".to_string()));
        }

        // status version must equal old + 1
        if instances.len() == 1 {
            let mut ins = instances[0].clone();
            ins.data.status_version += 1;
            ins.data.thing = to.clone();
            rtn.push(ins);
        }
        return Ok(rtn);
    }

    // all biz must same to "to"
    for mut r in instances {
        let mut instance = r.clone();
        instance.data.thing = to.clone();
        rtn.push(instance);
    }

    Ok(rtn)
}

pub type ConvertService = ConvertTaskImpl<PlanService, DeliveryService>;
