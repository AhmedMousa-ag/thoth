// use crate::{operations::planner::charts::structs::Steps, structs::traits::EncodingDecoding};
// use serde::{ser::{Serialize, SerializeStruct, Serializer}};

// impl Serialize for Steps {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Steps", 8)?;
//         s.serialize_field("operation_id", &self.operation_id)?;
//         s.serialize_field("step_id", &self.step_id)?;
//         s.serialize_field("node_id", &self.node_id)?;
//         s.serialize_field("x", &self.x)?;
//         s.serialize_field("y", &self.y)?;
//         s.serialize_field("result", &self.result)?;
//         let mut next_step = None;
//         let mut prev_step = None;
//         if let Some(n_step)=self.next_step.as_ref(){
//             //The reason for assigning the step id is to avoid recurrsion.
//             next_step = Some(n_step.borrow().step_id.clone());
//         }
//         if let Some(p_step)=self.prev_step.as_ref(){
//             //The reason for assigning the step id is to avoid recurrsion.
//             prev_step = Some(p_step.borrow().step_id.clone());
//         }
//         s.serialize_field("next_step", &next_step)?;
//         s.serialize_field("prev_step", &prev_step)?;
//         s.serialize_field("use_prev_res", &self.use_prev_res)?;
//         s.serialize_field("extra_info", &self.extra_info)?;
//         s.end()
//     }
// }

// impl EncodingDecoding for Steps {
//     fn decode_bytes(bytes: &[u8]) -> Self {
//         let str_msg = String::from_utf8(bytes.to_vec()).unwrap();
//         serde_json::from_str(&str_msg).unwrap()
//     }
//     fn encode_bytes(&self) -> Vec<u8> {
//         serde_json::to_string(self).unwrap().into()
//     }

// }
