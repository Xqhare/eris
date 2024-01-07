# Eris: System Performance Monitoring Service

Eris is a simple and efficient system performance monitoring service designed to log and analyze system performance metrics under load. Named after the ancient Greek goddess of chaos and discord, Eris provides insights into your system's behavior during high-stress situations, identifying potential bottlenecks or performance issues

## Features / Roadmap

- **Real-time Monitoring:** Eris continuously collects and logs key system metrics, providing real-time insights into the performance of your system.
- **Customizable Metrics:** Tailor Eris to your specific needs by selecting and monitoring the metrics most relevant to your application or infrastructure.
- **Scalability:** Eris is designed to handle high loads and scale with your system, ensuring reliable performance monitoring even in dynamic environments.
- **User-Friendly Interface:** A clean and intuitive interface allows users to easily navigate through performance metrics and identify potential issues.
- **Alerting:** Implement alerting mechanisms to notify users of performance anomalies or thresholds exceeding predetermined limits.
- **Data Logging:** Eris logs system performance data to a persistent storage location, enabling analysis and trend identification.

### What is saved to disc?

Currently these parameters are saved for every process:

```
pub struct Proc {
    pub name: String,
    pub pid: Pid,
    pub parent_name: String,
    pub parent_pid: Pid,
    pub cpu_usage_per: f32,
    pub date: String,
    pub vir_mem: u64,
    pub total_disc_read: u64,
    pub total_disc_write: u64,
    pub run_time: u64,
    pub usr_id: String,
}
```

## Eris: A Name Rooted in Chaos and Harmony

In the realm of ancient Greek mythology, Eris, the goddess of strife and discord, stands as a paradoxical figure. While often associated with chaos and destruction, she also played a role in instigating competition and driving innovation. This duality of Eris's nature aligns perfectly with the purpose of Eris, the system performance monitoring service. Eris is designed to uncover the underlying causes of performance issues, the root of chaos within a system. 

Just as Eris's golden apple ignited a conflict between goddesses, Eris's monitoring data can reveal hidden flaws or inefficiencies that, if left unchecked, could lead to system failures or performance bottlenecks.

The name Eris, therefore, not only reflects the service's ability to detect and diagnose performance problems but also captures its potential to drive positive change within a system. Just as Eris's intervention in the Trojan War led to the construction of the iconic Trojan Horse, Eris's insights can guide system administrators to optimize their systems and achieve remarkable performance improvements.

In essence, Eris is not merely a tool for detecting and resolving performance issues; it is a catalyst for innovation and efficiency. By embracing the duality of Eris, we can harness the power of chaos to achieve harmony and unlock the full potential of our systems.

## Acknowledgments
Thanks to the open-source community for providing invaluable tools and libraries.
Used in this project:
- [sysinfo](https://crates.io/crates/sysinfo)
- [chrono](https://crates.io/crates/chrono)
- [json](https://crates.io/crates/json)
