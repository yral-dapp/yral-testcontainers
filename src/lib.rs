pub mod metadata {
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        Image,
    };

    include!(concat!(env!("OUT_DIR"), "/meta/yral_metadata.rs"));

    pub const REST_PORT: ContainerPort = ContainerPort::Tcp(8001);
    pub const GRPC_PORT_TCP: ContainerPort = ContainerPort::Udp(8000);
    pub const GRPC_PORT_UDP: ContainerPort = ContainerPort::Udp(8000);

    pub struct YralMetadata;

    impl Image for YralMetadata {
        fn name(&self) -> &str {
            NAME
        }

        fn tag(&self) -> &str {
            TAG
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::millis(1000)]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[REST_PORT, GRPC_PORT_TCP, GRPC_PORT_UDP]
        }
    }
}

pub mod backend {
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        Image,
    };

    include!(concat!(env!("OUT_DIR"), "/meta/yral_backend.rs"));

    pub const AGENT_PORT: ContainerPort = ContainerPort::Tcp(4943);

    pub struct YralBackend;

    impl Image for YralBackend {
        fn name(&self) -> &str {
            NAME
        }

        fn tag(&self) -> &str {
            TAG
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::message_on_stdout("Initialized replica.")]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[AGENT_PORT]
        }
    }
}
