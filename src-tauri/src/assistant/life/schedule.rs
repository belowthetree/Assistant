use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleItem {
    pub content: String,
    pub date: DateTime<Local>
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Schedule {
    pub schedules: Vec<ScheduleItem>,
    pub daily_schedules: Vec<ScheduleItem>
}

impl Schedule {
    pub fn new()->Self {
        Self {
            schedules: Vec::new(),
            daily_schedules: Vec::new(),
        }
    }

    pub fn get_daily_schedules_string(&self)->String {
        let mut ret = r#"
## 日程安排
这是 json 格式的日程安排，可能为空："#.into();
        ret += serde_json::to_string(&self.daily_schedules).unwrap().as_str();
        // system info
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        ret += format!(r#"## 基础信息
时间：{}
## 操作系统
system:
// RAM and swap information:
total memory: {} bytes
used memory : {} bytes
total swap  : {} bytes
used swap   : {} bytes

// Display system information:
System name:             {:?}
System kernel version:   {:?}
System OS version:       {:?}
System host name:        {:?}

// Number of CPUs:
NB CPUs: {}
"#, Local::now(), sys.total_memory(), sys.used_memory(), sys.total_memory(),
            sys.used_memory(), System::name(), System::kernel_version(), System::os_version(), System::host_name(), sys.cpus().len()).as_str();
        ret
    }
}